use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    #[serde(rename = "cpe_id")]
    pub cpe_id: String,
    pub id: String,
    pub url: String,
    #[serde(rename = "req_time")]
    pub req_time: String,
    #[serde(rename = "method_type")]
    pub method_type: String,
    pub domain: String,
    #[serde(rename = "process_name")]
    pub process_name: String,
    pub risk_level: i32,
    pub ip: String,
    pub mac: String,
    pub host_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BehaviorLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub proc: String,
    pub op_time: String,
    #[serde(rename = "cpe_id")]
    pub cpe_id: String,
    pub op_type: String,
    pub detail: String,
    pub risk_level: i32,
    pub host_id: String,
    pub mac: String,
    pub ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenshotLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub capture_time: String,
    #[serde(rename = "cpe_id")]
    pub cpe_id: String,
    #[serde(rename = "image_path")]
    pub image_path: String,
    pub ocr_text: Option<String>,
    #[serde(rename = "is_sensitive")]
    pub risk_level: i32,
    pub app_name: String,
    pub image_hash: String,
    pub host_id: String,
    pub mac: String,
    pub ip: String,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub pin: String,
    pub host_id: String,
    pub cpe_id: String,
    pub mac: String,
    pub ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    pub server_logic_clock: Option<u64>,
    pub server_time: Option<u64>,     // Sync field
    #[serde(default)]
    pub need_update: bool,            // Sync field
    #[serde(default)]
    pub commands: Vec<HeartbeatCommand>,        // Sync field
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatCommand {
    pub command_id: String,
    pub op_type: String, // e.g., "reboot", "lock", "config"
    pub payload: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PopNode {
    pub pop_id: String,
    pub name: String,
    pub latency_hint: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub has_update: bool,
    pub version: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PolicyConfig {
    #[serde(default)]
    pub process_blacklist: Vec<String>,
    #[serde(default)]
    pub app_blacklist: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<PolicyConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertInfo {
    pub cert_pem: String,
}
