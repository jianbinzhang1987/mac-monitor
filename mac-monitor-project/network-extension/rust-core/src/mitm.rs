use crate::clock::LogicalClock;
use chrono::{Local, TimeZone};
use interprocess::local_socket::LocalSocketStream;
use lazy_static::lazy_static;
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

static LOG_COUNTER: AtomicU64 = AtomicU64::new(0);
lazy_static! {
    static ref GLOBAL_DEVICE_INFO: Mutex<DeviceInfo> = Mutex::new(DeviceInfo::default());
    static ref GLOBAL_AUDIT_POLICY: Mutex<AuditPolicy> = Mutex::new(AuditPolicy::default());
}

#[derive(Debug, Clone, Default)]
pub struct DeviceInfo {
    pub pin_number: String,
    pub ip: String,
    pub mac: String,
    pub cpe_id: String,
    pub host_id: String,
}

/// SSL MitM 代理，用于拦截和审计 HTTPS 流量
pub struct MitmProxy {
    root_ca: Certificate,
    cert_cache: HashMap<String, Arc<ServerConfig>>,
}

impl MitmProxy {
    pub fn new() -> Self {
        let mut params = CertificateParams::default();
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, "Mac Monitor Root CA");
        params.distinguished_name.push(DnType::OrganizationName, "Mac Monitor");
        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);

        let root_ca = Certificate::from_params(params).expect("Failed to generate root CA");

        Self {
            root_ca,
            cert_cache: HashMap::new(),
        }
    }

    /// 获取根 CA 证书 PEM（用于安装到系统信任链）
    pub fn get_root_ca_pem(&self) -> String {
        self.root_ca.serialize_pem().expect("Failed to serialize CA")
    }

    /// 为目标域名动态生成伪造证书
    pub fn generate_cert_for_domain(&mut self, domain: &str) -> Result<Arc<ServerConfig>> {
        if let Some(config) = self.cert_cache.get(domain) {
            return Ok(config.clone());
        }

        let mut params = CertificateParams::default();
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, domain);
        params.subject_alt_names = vec![rcgen::SanType::DnsName(domain.to_string())];

        let cert = Certificate::from_params(params)?;
        let cert_der = cert.serialize_der()?;
        let key_der = cert.serialize_private_key_der();

        let certs = vec![CertificateDer::from(cert_der)];
        let key = PrivateKeyDer::try_from(key_der)?;

        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| format!("Failed to build server config: {}", e))?;

        let config_arc = Arc::new(config);
        self.cert_cache.insert(domain.to_string(), config_arc.clone());

        Ok(config_arc)
    }



    /// 解析 HTTP 请求，提取审计所需信息
    pub fn parse_http_request(&self, data: &[u8]) -> Result<HttpRequest> {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        match req.parse(data) {
            Ok(httparse::Status::Complete(body_offset)) => {
                let method = req.method.unwrap_or("UNKNOWN").to_string();
                let path = req.path.unwrap_or("/").to_string();
                let version = req.version.unwrap_or(1);

                let mut header_map = HashMap::new();
                for header in req.headers {
                    if let Ok(value) = std::str::from_utf8(header.value) {
                        header_map.insert(header.name.to_string(), value.to_string());
                    }
                }

                let body = if body_offset < data.len() {
                    data[body_offset..].to_vec()
                } else {
                    vec![]
                };

                Ok(HttpRequest {
                    method,
                    path,
                    version,
                    headers: header_map,
                    body,
                })
            }
            _ => Err("Failed to parse HTTP request".into()),
        }
    }

    fn send_log_to_service(&self, log: &AuditLog) -> Result<()> {
        let socket_path = "/tmp/mac_monitor_audit.sock";
        let mut stream = LocalSocketStream::connect(socket_path)
            .map_err(|e| format!("IPC Connect error: {}", e))?;

        let command = serde_json::json!({
            "command": "log_traffic",
            "payload": log
        });

        let data = serde_json::to_string(&command)?;
        stream.write_all(data.as_bytes())?;

        Ok(())
    }
}

fn next_log_id() -> String {
    let seq = LOG_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}-{}", LogicalClock::now_ms(), seq)
}

pub fn set_device_info(info: DeviceInfo) {
    let mut current = GLOBAL_DEVICE_INFO.lock().unwrap();
    *current = info;
}

fn current_device_info() -> DeviceInfo {
    GLOBAL_DEVICE_INFO.lock().unwrap().clone()
}

pub fn set_audit_policy_json(json: &str) -> Result<()> {
    let policy: AuditPolicy = serde_json::from_str(json)?;
    let mut current = GLOBAL_AUDIT_POLICY.lock().unwrap();
    *current = policy;
    Ok(())
}

fn should_log_request(domain: &str, url: &str) -> bool {
    let policy = GLOBAL_AUDIT_POLICY.lock().unwrap();
    let domain_lower = domain.to_ascii_lowercase();

    // 优先检查目标域名列表（如果配置了，则只记录匹配的域名）
    if !policy.target_domains.is_empty() {
        let mut matched = false;
        for entry in &policy.target_domains {
            if !entry.enabled {
                continue;
            }
            let target = entry.domain.trim().to_ascii_lowercase();
            if target.is_empty() {
                continue;
            }

            // 支持通配符匹配: *.google.com 匹配 www.google.com, mail.google.com 等
            if target.starts_with("*.") {
                let suffix = &target[2..]; // 去掉 "*."
                if domain_lower == suffix || domain_lower.ends_with(&format!(".{}", suffix)) {
                    matched = true;
                    break;
                }
            } else if domain_lower == target || domain_lower.ends_with(&format!(".{}", target)) {
                matched = true;
                break;
            }
        }

        if !matched {
            return false; // 不在目标域名列表中，不记录
        }
    }

    // 检查白名单（白名单中的域名不记录）
    if !policy.white_domains.is_empty() {
        for entry in &policy.white_domains {
            let item = entry.domain.trim().to_ascii_lowercase();
            if item.is_empty() {
                continue;
            }
            if domain_lower == item || domain_lower.ends_with(&format!(".{}", item)) {
                return false;
            }
        }
    }

    let _ = url;
    true
}

pub fn response_body_for_url(url: &str, body: &[u8]) -> Option<String> {
    if body.is_empty() {
        return None;
    }
    let policy = GLOBAL_AUDIT_POLICY.lock().unwrap();
    for config in &policy.website_rsp_config_array {
        if config.url.is_empty() {
            continue;
        }
        if url.contains(&config.url) {
            let max_len = config.rspbodylength;
            let slice = if max_len > 0 && body.len() > max_len {
                &body[..max_len]
            } else {
                body
            };
            return Some(String::from_utf8_lossy(slice).to_string());
        }
    }
    None
}

fn format_logical_time(timestamp_secs: i64) -> String {
    Local
        .timestamp_opt(timestamp_secs, 0)
        .single()
        .unwrap_or_else(Local::now)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: u8,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditLog {
    pub pin_number: String,
    pub id: String,
    pub url: String,
    pub req_time: String,
    pub resp_time: String,
    pub method_type: String,
    pub ip: String,
    pub mac: String,
    pub cpe_id: String,
    pub host_id: String,
    pub status_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body: Option<String>,

    pub process_name: String,
}

impl MitmProxy {
    // ... existing implementation ...

    pub fn handle_https_connection(&mut self, domain: &str, client_stream: &[u8], src_port: u16) -> Result<Vec<u8>> {
        let _server_config = self.generate_cert_for_domain(domain)?;
        let device_info = current_device_info();

        let parsed = self.parse_http_request(client_stream).ok();
        let (method, path, body) = if let Some(req) = parsed {
            (req.method, req.path, req.body)
        } else {
            ("UNKNOWN".to_string(), "/".to_string(), Vec::new())
        };

        let url = if path.starts_with("http://") || path.starts_with("https://") {
            path.clone()
        } else {
            format!("https://{}{}", domain, path)
        };

        // Check if we need to log
        if !should_log_request(domain, &url) {
            return Ok(client_stream.to_vec());
        }

        let req_time = format_logical_time(LogicalClock::now());
        let resp_time = format_logical_time(LogicalClock::now());

        // Resolve process name
        let process_name = resolve_process_name(src_port).unwrap_or_else(|| "unknown".to_string());

        let log = AuditLog {
            pin_number: device_info.pin_number,
            id: next_log_id(),
            url: url.clone(),
            req_time,
            resp_time,
            method_type: method,
            ip: device_info.ip,
            mac: device_info.mac,
            cpe_id: device_info.cpe_id,
            host_id: device_info.host_id,
            status_code: "200".to_string(),
            request_body: if body.is_empty() {
                None
            } else {
                Some(String::from_utf8_lossy(&body).to_string())
            },
            response_body: None,
            process_name,
        };

        if let Err(e) = self.send_log_to_service(&log) {
            log::error!("Failed to send log via IPC: {}", e);
        }

        Ok(client_stream.to_vec())
    }
}

fn resolve_process_name(port: u16) -> Option<String> {
    use std::process::Command;
    // Attempt to use lsof to find the process using the port
    // Command: lsof -i :<port> -sTCP:ESTABLISHED -F c
    // Output format with -F c:
    // p12345
    // cProcessName
    
    // Note: lsof might require sudo or might not work in sandbox. 
    // Trying best effort.
    let output = Command::new("lsof")
        .args(&["-i", &format!(":{}", port), "-sTCP:ESTABLISHED", "-F", "c"])
        .output()
        .ok()?;

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        for line in s.lines() {
            if line.starts_with('c') {
                return Some(line[1..].to_string());
            }
        }
    }
    None
}

#[derive(Debug, Clone, Default, Deserialize)]
struct AuditPolicy {
    #[serde(default)]
    white_domains: Vec<WhiteDomain>,
    #[serde(default)]
    target_domains: Vec<TargetDomain>,
    #[serde(default)]
    website_rsp_config_array: Vec<ResponseConfig>,
}

#[derive(Debug, Clone, Deserialize)]
struct WhiteDomain {
    domain: String,
    #[serde(default)]
    id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct TargetDomain {
    domain: String,
    #[serde(default)]
    enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ResponseConfig {
    #[serde(default)]
    rspbodylength: usize,
    #[serde(default)]
    url: String,
}
