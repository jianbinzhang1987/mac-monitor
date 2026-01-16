use std::sync::{Arc, RwLock};
use sysinfo::System;
use crate::db::Database;
use crate::models::{PolicyConfig, BehaviorLog};
use chrono::Local;
use std::path::Path;

pub struct Scanner {
    db: Arc<Database>,
    policy: Arc<RwLock<PolicyConfig>>,
    sys: System,
    device_info: crate::models::DeviceInfo,
}

impl Scanner {
    pub fn new(db: Arc<Database>, policy: Arc<RwLock<PolicyConfig>>, device_info: crate::models::DeviceInfo) -> Self {
        Self {
            db,
            policy,
            sys: System::new_all(),
            device_info,
        }
    }

    pub async fn scan(&mut self) {
        log::info!("[Scanner] Starting security scan... checking processes and applications");

        // 刷新系统信息
        self.sys.refresh_processes();

        let (process_blacklist, app_blacklist) = {
            let p = self.policy.read().unwrap();
            (p.process_blacklist.clone(), p.app_blacklist.clone())
        };

        // 1. 扫描异常运行进程 (黑名单判定)
        for (pid, process) in self.sys.processes() {
            let name = process.name().to_lowercase();
            for black_name in &process_blacklist {
                if name.contains(&black_name.to_lowercase()) {
                    self.report_anomaly(
                        &name,
                        "AbnormalProcess",
                        &format!("Detected running blacklisted process: {} (PID: {})", name, pid)
                    ).await;
                    // 命中一个黑名单后跳过当前进程的其他黑名单检查
                    break;
                }
            }
        }

        // 2. 扫描异常安装程序 (根据管理端下发的具体名单)
        let mut app_dirs = vec!["/Applications".to_string(), "/Users/Shared".to_string()];
        if let Some(home) = std::env::var_os("HOME") {
            let home_path = std::path::PathBuf::from(home);
            app_dirs.push(home_path.join("Applications").to_string_lossy().into_owned());
        }

        for black_app in &app_blacklist {
            let black_app_lower = black_app.to_lowercase();
            for dir in &app_dirs {
                // 1. 检查精确路径: /Applications/Clash.app
                let app_path = format!("{}/{}.app", dir, black_app);
                if Path::new(&app_path).exists() {
                    self.report_anomaly(
                        black_app,
                        "AbnormalAppInstalled",
                        &format!("Detected blacklisted application installed at: {}", app_path)
                    ).await;
                    continue;
                }

                // 2. 模糊匹配目录下的其他项: /Applications/Clash for Windows.app
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let file_name = entry.file_name().to_string_lossy().to_lowercase();
                        if file_name.contains(&black_app_lower) && file_name.ends_with(".app") {
                            self.report_anomaly(
                                &file_name,
                                "AbnormalAppInstalled",
                                &format!("Detected suspicious application matching blacklist: {:?}", entry.path())
                            ).await;
                        }
                    }
                }
            }
        }
    }

    async fn report_anomaly(&self, proc: &str, op_type: &str, detail: &str) {
        // 使用 log::warn 记录报警信息
        log::warn!("🚨 [Scanner ALARM] Type: {}, Detail: {}", op_type, detail);

        let log = BehaviorLog {
            id: None,
            proc: proc.to_string(),
            op_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            cpe_id: self.device_info.cpe_id.clone(),
            op_type: op_type.to_string(),
            detail: detail.to_string(),
            risk_level: 2, // 2 = 高风险
            host_id: self.device_info.host_id.clone(),
            mac: self.device_info.mac.clone(),
            ip: self.device_info.ip.clone(),
        };

        if let Err(e) = self.db.save_behavior_log(&log).await {
            log::error!("Failed to save anomaly log: {}", e);
        } else {
            log::info!("🚨 Anomaly detected and logged: {} - {}", op_type, detail);
        }
    }
}
