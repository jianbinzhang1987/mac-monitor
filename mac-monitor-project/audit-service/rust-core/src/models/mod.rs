use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BehaviorLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub proc: String,
    pub op_time: String,
    pub pin: String,
    pub op_file: String,
    pub op_type: String, // 1 修改文件 2 完整性校验检查 3剪切板 4共享网络 5共享热点 6网络代理 7wlan设备插拔行为
    pub op_ret: String,
    pub op_reason: String,
    pub host_id: String,
    pub cpe_id: String,
    pub mac: String,
    pub ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenshotLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub pin: String,
    pub capture_time: String,
    pub app_name: String,
    pub window_title: String,
    pub image_path: String, // 加密后的图片存储路径
    pub image_hash: String, // 图片哈希值，用于去重
    pub is_sensitive: bool, // 是否包含敏感信息（经过OCR检测）
    pub ocr_text: Option<String>, // OCR识别的文本（仅用于敏感词匹配，不上报）
    pub host_id: String,
    pub cpe_id: String,
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
