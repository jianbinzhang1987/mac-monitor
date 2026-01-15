pub mod sync;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Uploader {
    client: Client,
    app_code: String,
    app_secret: String,
    base_url: String,
    visit_token: Arc<Mutex<Option<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<TokenData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    #[serde(rename = "visit-token")]
    pub visit_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub code: i32,
    pub msg: String,
    pub url: Option<String>,
    pub fileName: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MonitorDeviceLogin {
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "deviceName")]
    pub device_name: String,
    #[serde(rename = "osVersion")]
    pub os_version: String,
    #[serde(rename = "appVersion")]
    pub app_version: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<LoginData>,
}

#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub token: String,
}

impl Uploader {
    pub fn new(app_code: &str, app_secret: &str, base_url: &str) -> Self {
        Self {
            client: Client::new(),
            app_code: app_code.to_string(),
            app_secret: app_secret.to_string(),
            base_url: base_url.to_string(),
            visit_token: Arc::new(Mutex::new(None)),
        }
    }

    async fn get_token(&self) -> Result<String, String> {
        {
            let token = self.visit_token.lock().unwrap();
            if let Some(t) = token.as_ref() {
                return Ok(t.clone());
            }
        }

        // 调用登录接口获取真实 Token
        let login_info = MonitorDeviceLogin {
            serial_number: "MAC_SN_123456".to_string(), // TODO: 获取真实序列号
            device_name: "Adolf's MacBook".to_string(),
            os_version: "macOS 15.1".to_string(),
            app_version: "0.1.0".to_string(),
        };

        let url = format!("{}/api/v1/login", self.base_url);
        let response = self.client.post(&url)
            .json(&login_info)
            .send()
            .await
            .map_err(|e| format!("Login request failed: {}", e))?;

        let res: LoginResponse = response.json().await.map_err(|e| format!("Failed to parse login response: {}", e))?;

        if res.code == 200 || res.code == 0 {
            if let Some(data) = res.data {
                let mut token_lock = self.visit_token.lock().unwrap();
                *token_lock = Some(data.token.clone());
                return Ok(data.token);
            }
        }

        Err(format!("Login failed: {}", res.msg))
    }

    fn generate_signature(&self, _timestamp: u64, _nonce: &str) -> String {
        "simple_sig".to_string()
    }

    pub async fn upload_data<T: Serialize>(&self, endpoint: &str, data: &T) -> Result<(), String> {
        let token = self.get_token().await?;
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(data)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(format!("Upload failed to {} with status: {}. Body: {}", url, status, body))
        }
    }

    pub async fn upload_file(&self, file_path: &str) -> Result<String, String> {
        let token = self.get_token().await?;
        let file_content = std::fs::read(file_path).map_err(|e| e.to_string())?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file.jpg")
            .to_string();

        let part = reqwest::multipart::Part::bytes(file_content)
            .file_name(file_name)
            .mime_str("image/jpeg")
            .map_err(|e| e.to_string())?;

        let form = reqwest::multipart::Form::new().part("file", part);

        let url = format!("{}/api/v1/upload/screenshot", self.base_url);
        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .multipart(form)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let res: UploadResponse = response.json().await.map_err(|e| e.to_string())?;
        if res.code == 200 || res.code == 0 {
            res.url.ok_or_else(|| "No URL in upload response".to_string())
        } else {
            Err(format!("Upload file failed: {}", res.msg))
        }
    }

    pub async fn get_server_time(&self) -> Result<u64, String> {
        let response = self.client.get(format!("{}/httpsaudit/zf/api/third/server/time", self.base_url))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        #[derive(Deserialize)]
        struct TimeResponse {
            data: TimeData,
        }
        #[derive(Deserialize)]
        struct TimeData {
            systime: String,
        }

        let res: TimeResponse = response.json().await.map_err(|e| e.to_string())?;
        // 尝试解析字符串时间为 Unix 时间戳，或直接要求后端返回数字
        res.data.systime.parse::<u64>().map_err(|_| "Invalid server time format".to_string())
    }

    pub async fn heartbeat(&self, current_version: &str) -> Result<crate::models::HeartbeatResponse, String> {
        let data = serde_json::json!({
            "app_version": current_version,
            "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        });
        
        let token = self.get_token().await?;
        let response = self.client.post(format!("{}/api/v1/heartbeat", self.base_url))
            .header("visit-token", token)
            .json(&data)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        response.json().await.map_err(|e| e.to_string())
    }

    pub async fn get_pop_list(&self) -> Result<Vec<crate::models::PopNode>, String> {
        let token = self.get_token().await?;
        let response = self.client.get(format!("{}/api/v1/pop/list", self.base_url))
            .header("visit-token", token)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        #[derive(Deserialize)]
        struct PopListResponse {
            data: Vec<crate::models::PopNode>,
        }

        let res: PopListResponse = response.json().await.map_err(|e| e.to_string())?;
        Ok(res.data)
    }

    pub async fn check_update(&self) -> Result<crate::models::UpdateInfo, String> {
        let response = self.client.get(format!("{}/api/v1/maintenance/update", self.base_url))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        #[derive(Deserialize)]
        struct UpdateResponse {
            data: crate::models::UpdateInfo,
        }

        let res: UpdateResponse = response.json().await.map_err(|e| e.to_string())?;
        Ok(res.data)
    }

    pub async fn get_config(&self) -> Result<crate::models::PolicyConfig, String> {
        let token = self.get_token().await?;
        let response = self.client.get(format!("{}/api/v1/config/policy", self.base_url))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let res: crate::models::ConfigResponse = response.json().await.map_err(|e| e.to_string())?;
        if res.code == 200 || res.code == 0 {
            res.data.ok_or_else(|| "No policy config in response".to_string())
        } else {
            Err(format!("Get config failed: {}", res.msg))
        }
    }

    pub async fn get_cert_info(&self) -> Result<crate::models::CertInfo, String> {
        let response = self.client.get(format!("{}/api/v1/maintenance/cert", self.base_url))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        #[derive(Deserialize)]
        struct CertResponse {
            data: crate::models::CertInfo,
        }

        let res: CertResponse = response.json().await.map_err(|e| e.to_string())?;
        Ok(res.data)
    }
}
