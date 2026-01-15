pub mod sync;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use libsm::sm3::hash::Sm3Hash;
use uuid::Uuid;
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

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let nonce = Uuid::new_v4().simple().to_string();
        let signature = self.generate_signature(timestamp, &nonce);

        let response = self.client.post(format!("{}/httpsaudit/zf/api/third/visit-token", self.base_url))
            .header("app-code", &self.app_code)
            .header("timestamp", timestamp.to_string())
            .header("nonce", &nonce)
            .header("signature", &signature)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let res: TokenResponse = response.json().await.map_err(|e| e.to_string())?;
        if let Some(data) = res.data {
            let mut token = self.visit_token.lock().unwrap();
            *token = Some(data.visit_token.clone());
            Ok(data.visit_token)
        } else {
            Err(format!("Failed to get token: {}", res.msg))
        }
    }

    fn generate_signature(&self, timestamp: u64, nonce: &str) -> String {
        let raw = format!("{}{}{}{}", self.app_code, self.app_secret, timestamp, nonce);
        let mut hash = Sm3Hash::new(raw.as_bytes());
        let digest = hash.get_hash();
        hex::encode(digest)
    }

    pub async fn upload_data<T: Serialize>(&self, endpoint: &str, data: &T) -> Result<(), String> {
        let token = self.get_token().await?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let nonce = Uuid::new_v4().simple().to_string();
        let signature = self.generate_signature(timestamp, &nonce);

        let response = self.client.post(format!("{}{}", self.base_url, endpoint))
            .header("visit-token", token)
            .header("app-code", &self.app_code)
            .header("timestamp", timestamp.to_string())
            .header("nonce", &nonce)
            .header("signature", &signature)
            .json(data)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Upload failed with status: {}", response.status()))
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
