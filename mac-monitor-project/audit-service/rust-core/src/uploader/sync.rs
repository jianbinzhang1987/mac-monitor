use std::sync::Arc;
use tokio::time::{self, Duration};
use crate::db::Database;
use crate::uploader::Uploader;

use crate::clock::LogicalClock;

pub struct SyncService {
    db: Arc<Database>,
    uploader: Arc<Uploader>,
    clock: Arc<LogicalClock>,
}

impl SyncService {
    pub fn new(db: Arc<Database>, uploader: Arc<Uploader>, clock: Arc<LogicalClock>) -> Self {
        Self { db, uploader, clock }
    }

    pub fn start(self) {
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(60)); // 每分钟同步一次
            loop {
                interval.tick().await;
                // 1. 执行心跳与较时
                if let Err(e) = self.do_heartbeat().await {
                    eprintln!("Heartbeat failed: {}", e);
                }

                // 2. 同步日志
                if let Err(e) = self.sync_logs().await {
                    eprintln!("Sync failed: {}", e);
                }
            }
        });
    }

    async fn do_heartbeat(&self) -> Result<(), String> {
        let res = self.uploader.heartbeat("0.1.0").await?;
        
        // 更新逻辑时钟
        self.clock.update_offset(res.server_time as i64);

        if res.need_update {
            println!("Policy update needed, fetching latest config...");
            // TODO: 调用 get_config 并更新本地数据库/内存策略
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
            match self.uploader.upload_data("/httpsaudit/zf/api/auditlog/upload", &log).await {
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
            match self.uploader.upload_data("/httpsaudit/zf/api/behavior/upload", &log).await {
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
        for log in screenshot_logs {
            // 截图通常需要上传图片文件 + 元数据，这里简化为只上传元数据
            // 实际场景可能需要先上传文件获取 URL，再上传日志
            match self.uploader.upload_data("/httpsaudit/zf/api/screenshot/upload", &log).await {
                Ok(_) => {
                    self.db.mark_screenshot_log_sent(&log.image_hash).await.map_err(|e| e.to_string())?;
                }
                Err(e) => {
                    eprintln!("Failed to upload screenshot log {}: {}", log.image_hash, e);
                }
            }
        }

        Ok(())
    }
}
