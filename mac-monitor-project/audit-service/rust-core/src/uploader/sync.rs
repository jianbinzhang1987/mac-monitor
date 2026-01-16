use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tokio::time::{self, Duration};
use crate::db::Database;
use crate::uploader::Uploader;
use crate::models::PolicyConfig;
use crate::scanner::Scanner;

use crate::clock::LogicalClock;

pub struct SyncService {
    db: Arc<Database>,
    uploader: Arc<Uploader>,
    clock: Arc<LogicalClock>,
    policy: Arc<RwLock<PolicyConfig>>,
    device_info: crate::models::DeviceInfo,
    screenshot_dir: String,
}

impl SyncService {
    pub fn new(
        db: Arc<Database>,
        uploader: Arc<Uploader>,
        clock: Arc<LogicalClock>,
        policy: Arc<RwLock<PolicyConfig>>,
        device_info: crate::models::DeviceInfo,
        screenshot_dir: String,
    ) -> Self {
        Self { db, uploader, clock, policy, device_info, screenshot_dir }
    }

    pub fn start(self) {
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(60)); // 每分钟同步一次
            let mut scanner = Scanner::new(self.db.clone(), self.policy.clone(), self.device_info.clone());
            loop {
                interval.tick().await;

                // 1. 执行扫描检测异常进程和程序
                scanner.scan().await;

                // 2. 执行心跳与较时
                if let Err(e) = self.do_heartbeat().await {
                    eprintln!("Heartbeat failed: {}", e);
                }

                // 3. 同步日志
                if let Err(e) = self.sync_logs().await {
                    eprintln!("Sync failed: {}", e);
                }
            }
        });
    }

    async fn do_heartbeat(&self) -> Result<(), String> {
        let res = self.uploader.heartbeat("0.1.0").await?;
        
        // 更新逻辑时钟
        self.clock.update_offset(res.server_time.unwrap_or(0) as i64);

        if res.need_update {
            println!("Policy update needed, fetching latest config...");
            match self.uploader.get_config().await {
                Ok(new_policy) => {
                    let mut p = self.policy.write().unwrap();
                    *p = new_policy;
                    println!("Policy updated: {} processes, {} apps blacklisted",
                        p.process_blacklist.len(), p.app_blacklist.len());
                }
                Err(e) => eprintln!("Failed to fetch policy: {}", e),
            }
        }

        for cmd in res.commands {
            println!("Received remote command: {} ({})", cmd.command_id, cmd.op_type);
            // TODO: 处理远程指令
        }

        Ok(())
    }

    async fn sync_logs(&self) -> Result<(), String> {
        // 1. 同步审计日志
        let audit_logs = self.db.get_unsent_audit_logs().await.map_err(|e| e.to_string())?;
        for log in audit_logs {
            match self.uploader.upload_data("/api/v1/log/audit", &log).await {
                Ok(_) => {
                    self.db.mark_audit_log_sent(&log.id).await.map_err(|e| e.to_string())?;
                }
                Err(e) => {
                    eprintln!("Failed to upload audit log {}: {}", log.id, e);
                }
            }
        }

        // 2. 同步行为日志
        let behavior_logs = self.db.get_unsent_behavior_logs().await.map_err(|e| e.to_string())?;
        for log in behavior_logs {
            // 注意：API 路径仅为示例，需根据实际接口文档调整
            match self.uploader.upload_data("/api/v1/log/behavior", &log).await {
                Ok(_) => {
                    if let Some(id) = log.id {
                         self.db.mark_behavior_log_sent(id).await.map_err(|e| e.to_string())?;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to upload behavior log {:?}: {}", log.id, e);
                }
            }
        }

        // 3. 同步截图日志
        let screenshot_logs = self.db.get_unsent_screenshot_logs().await.map_err(|e| e.to_string())?;
        for mut log in screenshot_logs {
            let resolved_path = self.resolve_screenshot_path(&log.image_path);
            if !Path::new(&resolved_path).exists() {
                eprintln!(
                    "Screenshot file missing after path resolution, marking as sent: {}",
                    log.image_path
                );
                self.db.mark_screenshot_log_sent(&log.image_hash).await.map_err(|e| e.to_string())?;
                continue;
            }

            if log.image_path != resolved_path {
                log.image_path = resolved_path;
            }
            // 3.1 首先上传真实的图片文件
            match self.uploader.upload_file(&log.image_path).await {
                Ok(remote_url) => {
                    // 3.2 替换为服务器端的 URL
                    let local_path = log.image_path.clone();
                    log.image_path = remote_url;

                    // 3.3 上传元数据
                    match self.uploader.upload_data("/api/v1/log/screenshot", &log).await {
                        Ok(_) => {
                            self.db.mark_screenshot_log_sent(&log.image_hash).await.map_err(|e| e.to_string())?;
                        }
                        Err(e) => {
                            eprintln!("Failed to upload screenshot metadata {}: {}", log.image_hash, e);
                            // 恢复本地路径，以便下次重试
                            log.image_path = local_path;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to upload screenshot file {}: {}", log.image_path, e);
                }
            }
        }

        Ok(())
    }

    fn resolve_screenshot_path(&self, image_path: &str) -> String {
        let path = Path::new(image_path);
        if path.exists() {
            return image_path.to_string();
        }

        let file_name = path.file_name().and_then(|n| n.to_str());
        if let Some(file_name) = file_name {
            let candidate: PathBuf = Path::new(&self.screenshot_dir).join(file_name);
            return candidate.to_string_lossy().to_string();
        }

        image_path.to_string()
    }
}
