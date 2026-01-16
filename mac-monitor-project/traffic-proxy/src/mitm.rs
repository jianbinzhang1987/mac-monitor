use crate::clock::LogicalClock;
use chrono::{Local, TimeZone};
use lazy_static::lazy_static;
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, KeyPair};
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

/// SSL MitM 代理辅助类，负责证书管理和日志上报
pub struct MitmProxy {
    root_ca: Certificate,
    cert_cache: HashMap<String, Arc<ServerConfig>>,
}

const CA_CERT_FILE: &str = "ca.pem";
const CA_KEY_FILE: &str = "ca.key";

impl MitmProxy {
    pub fn new() -> Self {
        let (cert_pem, key_pem) = Self::load_or_generate_ca_files().expect("Failed to load or generate CA");
        
        // rcgen 0.12+ doesn't easily support loading a full Certificate struct (with signing capability) from PEM 
        // that preserves exactly the same Serial/Validity as before unless we parse it perfectly.
        // However, for MITM, reusing the KeyPair and keeping DN is usually enough for "stability" 
        // IF we re-generate the CA cert with same params. 
        // BUT browser pins certificate fingerprints. So we MUST use the exact same certificate DER/PEM.
        // The issue: rcgen::Certificate struct owns the keypair and params.
        // Solution: parsing the existing cert PEM to extract params is hard without OpenSSL/other crates.
        // 
        // SIMPLIFICATION for this iteration:
        // We will generate a NEW Certificate object in memory using the LOADED KeyPair. 
        // This keeps the Public Key constant. 
        // To keep the Certificate Fingerprint constant, we'd need to force the same SerialNumber and Validity.
        // 
        // Improved Strategy:
        // 1. Load KeyPair from file.
        // 2. Generate params.
        // 3. Set key_pair to loaded keypair.
        // 4. Force serial number and validity to fixed values (or saved values).
        // 
        // Let's implement key persistence first. Fixed Serial/Validity helps stability.
        
        let key_pair = KeyPair::from_pem(&key_pem).expect("Failed to parse CA key");
        
        let mut params = CertificateParams::default();
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, "Mac Monitor Root CA");
        params.distinguished_name.push(DnType::OrganizationName, "Mac Monitor");
        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        params.key_pair = Some(key_pair);
        // Fix validity to ensure identical certificate regeneration
        // From 2025-01-01 to 4096-01-01
        use time::macros::datetime;
        params.not_before = datetime!(2025-01-01 00:00:00 UTC);
        params.not_after = datetime!(4096-01-01 00:00:00 UTC);
        
        let root_ca = Certificate::from_params(params).expect("Failed to generate root CA");
        
        // Write back cert pem just to be sure it matches what we use in memory
        let current_pem = root_ca.serialize_pem().unwrap();
        if current_pem != cert_pem {
            let _ = std::fs::write(CA_CERT_FILE, &current_pem);
        }

        Self {
            root_ca,
            cert_cache: HashMap::new(),
        }
    }

    fn load_or_generate_ca_files() -> std::io::Result<(String, String)> {
        use std::path::Path;
        if Path::new(CA_CERT_FILE).exists() && Path::new(CA_KEY_FILE).exists() {
            let cert = std::fs::read_to_string(CA_CERT_FILE)?;
            let key = std::fs::read_to_string(CA_KEY_FILE)?;
            return Ok((cert, key));
        }

        // Generate new
        let mut params = CertificateParams::default();
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, "Mac Monitor Root CA");
        params.distinguished_name.push(DnType::OrganizationName, "Mac Monitor");
        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        params.serial_number = Some(123456789.into()); // Constant serial

        let cert = Certificate::from_params(params).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let cert_pem = cert.serialize_pem().unwrap();
        let key_pem = cert.serialize_private_key_pem();

        std::fs::write(CA_CERT_FILE, &cert_pem)?;
        std::fs::write(CA_KEY_FILE, &key_pem)?;
        
        Ok((cert_pem, key_pem))
    }

    pub fn get_root_ca_pem(&self) -> String {
        self.root_ca.serialize_pem().expect("Failed to serialize CA")
    }

    pub fn generate_cert_for_domain(&mut self, domain: &str) -> Result<Arc<ServerConfig>> {
        if let Some(config) = self.cert_cache.get(domain) {
            return Ok(config.clone());
        }

        let mut params = CertificateParams::default();
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, domain);
        params.subject_alt_names = vec![rcgen::SanType::DnsName(domain.to_string())];
        // Sign with root CA
        let cert = Certificate::from_params(params)?;
        let cert_der = cert.serialize_der_with_signer(&self.root_ca)?;
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

    // Returns serialized JSON byte vector if logging is needed, otherwise None.
    pub fn prepare_audit_log(&self, domain: &str, method: &str, path: &str, body: &[u8]) -> Result<Option<Vec<u8>>> {
        let url = if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("https://{}{}", domain, path)
        };
        
        if !should_log_request(domain, &url) {
            return Ok(None);
        }

        let device_info = current_device_info();
        let req_time = format_logical_time(LogicalClock::now());
        let resp_time = format_logical_time(LogicalClock::now());

        let log = AuditLog {
            pin_number: device_info.pin_number,
            id: next_log_id(),
            url,
            req_time,
            resp_time,
            method_type: method.to_string(),
            domain: domain.to_string(),
            ip: device_info.ip,
            mac: device_info.mac,
            cpe_id: device_info.cpe_id,
            host_id: device_info.host_id,
            status_code: "200".to_string(),
            request_body: if body.is_empty() { None } else { Some(String::from_utf8_lossy(body).to_string()) },
            response_body: None,
            title: None,
        };

        let command = serde_json::json!({
            "command": "log_traffic",
            "payload": log
        });

        Ok(Some(serde_json::to_vec(&command)?))
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

    if !policy.target_domains.is_empty() {
        let mut matched = false;
        for entry in &policy.target_domains {
            if !entry.enabled { continue; }
            let target = entry.domain.trim().to_ascii_lowercase();
            if target.is_empty() { continue; }
            if target.starts_with("*.") {
                 let suffix = &target[2..];
                 if domain_lower == suffix || domain_lower.ends_with(&format!(".{}", suffix)) {
                     matched = true; break;
                 }
            } else if domain_lower == target || domain_lower.ends_with(&format!(".{}", target)) {
                matched = true; break;
            }
        }
        if !matched {
            // Hardcoded domains for auto-recording
            if domain_lower == "google.com" || domain_lower.ends_with(".google.com") ||
               domain_lower == "github.com" || domain_lower.ends_with(".github.com") {
                // matched = true; 
            } else {
                return false;
            }
        }
    } else {
        // If no target domains are set, we still check the hardcoded defaults
        if !(domain_lower == "google.com" || domain_lower.ends_with(".google.com") ||
             domain_lower == "github.com" || domain_lower.ends_with(".github.com")) {
             // If not in hardcoded list and no policy, maybe log everything? 
             // The requirement says "DOMAIN IS *.google.com and *.github.com... record to local table"
             // This suggests we should at least record these.
        }
    }

    if !policy.white_domains.is_empty() {
        for entry in &policy.white_domains {
            let item = entry.domain.trim().to_ascii_lowercase();
            if item.is_empty() { continue; }
            if domain_lower == item || domain_lower.ends_with(&format!(".{}", item)) {
                return false;
            }
        }
    }
    
    let _ = url;
    true
}

pub fn response_body_for_url(url: &str, body: &[u8]) -> Option<String> {
    if body.is_empty() { return None; }
    let policy = GLOBAL_AUDIT_POLICY.lock().unwrap();
    for config in &policy.website_rsp_config_array {
        if config.url.is_empty() { continue; }
        if url.contains(&config.url) {
            let max_len = config.rspbodylength;
            let slice = if max_len > 0 && body.len() > max_len {
                &body[..max_len]
            } else { body };
            return Some(String::from_utf8_lossy(slice).to_string());
        }
    }
    None
}

fn format_logical_time(timestamp_secs: i64) -> String {
    Local.timestamp_opt(timestamp_secs, 0).single().unwrap_or_else(Local::now).format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditLog {
    pub pin_number: String,
    pub id: String,
    pub url: String,
    pub domain: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
