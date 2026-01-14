use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{ClientConfig, ServerConfig};
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, KeyPair};
use std::collections::HashMap;
use std::sync::Arc;
use serde::Serialize;
use interprocess::local_socket::LocalSocketStream;
use std::io::Write;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// SSL MitM代理，用于拦截和审计HTTPS流量
pub struct MitmProxy {
    root_ca: Certificate,
    cert_cache: HashMap<String, Arc<ServerConfig>>,
}

impl MitmProxy {
    pub fn new() -> Self {
        // 生成根CA证书
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

    /// 获取根CA证书的PEM格式（用于安装到系统信任链）
    pub fn get_root_ca_pem(&self) -> String {
        self.root_ca.serialize_pem().expect("Failed to serialize CA")
    }

    /// 为目标域名动态生成伪造证书
    pub fn generate_cert_for_domain(&mut self, domain: &str) -> Result<Arc<ServerConfig>> {
        // 检查缓存
        if let Some(config) = self.cert_cache.get(domain) {
            return Ok(config.clone());
        }

        // 生成新证书
        let mut params = CertificateParams::default();
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, domain);
        params.subject_alt_names = vec![
            rcgen::SanType::DnsName(domain.to_string()),
        ];

        let cert = Certificate::from_params(params)?;
        let cert_der = cert.serialize_der()?;
        let key_der = cert.serialize_private_key_der();

        // 构建ServerConfig
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

    /// 处理HTTPS连接的MitM逻辑
    pub fn handle_https_connection(
        &mut self,
        domain: &str,
        client_stream: &[u8],
    ) -> Result<Vec<u8>> {
        // 1. 获取或生成该域名的证书
        let _server_config = self.generate_cert_for_domain(domain)?;

        // 2. 建立双向TLS连接
        //    Client <--TLS--> Proxy <--TLS--> Server

        // 3. 在内存中解密HTTP流量
        //    解析HTTP请求/响应，提取敏感信息（URL、Headers、Body）

        // 4. 根据审计规则判断是否记录、过滤或阻断

        log::info!("MitM: Intercepting HTTPS traffic for domain: {}", domain);

        // 构造模拟日志 (实际应从解析结果中获取)
        let log = AuditLog {
            timestamp: chrono::Local::now().timestamp(),
            source_ip: "127.0.0.1".to_string(),
            dest_ip: "0.0.0.0".to_string(),
            domain: domain.to_string(),
            url: format!("https://{}/", domain),
            method: "GET".to_string(),
            request_size: client_stream.len(),
            response_size: 0,
            status_code: 200,
        };

        // 发送日志到审计服务
        if let Err(e) = self.send_log_to_service(&log) {
            log::error!("Failed to send log via IPC: {}", e);
        }

        // 临时返回原始数据（实际应返回重新加密的数据）
        Ok(client_stream.to_vec())
    }

    fn send_log_to_service(&self, log: &AuditLog) -> Result<()> {
        let socket_path = "/tmp/mac_monitor_audit.sock";
        let mut stream = LocalSocketStream::connect(socket_path)
            .map_err(|e| format!("IPC Connect error: {}", e))?;

        // 构造 IPC 消息格式
        let command = serde_json::json!({
            "command": "log_traffic",
            "payload": log
        });

        let data = serde_json::to_string(&command)?;
        stream.write_all(data.as_bytes())?;

        Ok(())
    }

    /// 解析HTTP请求，提取审计所需信息
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
    pub timestamp: i64,
    pub source_ip: String,
    pub dest_ip: String,
    pub domain: String,
    pub url: String,
    pub method: String,
    pub request_size: usize,
    pub response_size: usize,
    pub status_code: u16,
}
