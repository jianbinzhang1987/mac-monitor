use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::sync::Arc;
use tokio::runtime::Handle;
use crate::db::Database;
use crate::uploader::Uploader;
use crate::models::AuditLog;

#[derive(Debug, Deserialize)]
struct IpcCommand {
    command: String,
    payload: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct IpcResponse {
    status: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<serde_json::Value>,
}

pub struct IpcServer {
    db: Arc<Database>,
    uploader: Arc<Uploader>,
    runtime_handle: Handle,
}

impl IpcServer {
    pub fn new(db: Arc<Database>, uploader: Arc<Uploader>, runtime_handle: Handle) -> Self {
        Self { db, uploader, runtime_handle }
    }

    pub fn start(self) {
        std::thread::spawn(move || {
            let name = "/tmp/mac_monitor_audit.sock";
            let _ = std::fs::remove_file(name);

            let listener = LocalSocketListener::bind(name)
                .expect("Failed to bind IPC socket");

            println!("IPC server listening on {}", name);

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        self.handle_client(stream);
                    }
                    Err(e) => {
                        eprintln!("IPC connection failed: {}", e);
                    }
                }
            }
        });
    }

    fn handle_client(&self, mut stream: LocalSocketStream) {
        let mut buffer = [0u8; 65536];
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let cmd_str = String::from_utf8_lossy(&buffer[..n]);
                println!("Received IPC command: {}", cmd_str);

                let response = match serde_json::from_str::<IpcCommand>(&cmd_str) {
                    Ok(cmd) => self.process_command(cmd),
                    Err(e) => IpcResponse {
                        status: "error".to_string(),
                        message: format!("Invalid JSON: {}", e),
                        payload: None,
                    },
                };

                let response_json = serde_json::to_string(&response).unwrap_or_else(|_| "{\"status\":\"error\"}".to_string());
                let _ = stream.write_all(response_json.as_bytes());
                let _ = stream.flush();
            }
            Ok(_) => println!("IPC client closed connection or sent empty data"),
            Err(e) => eprintln!("Failed to read from IPC stream: {}", e),
        }
        // stream is dropped here, closing the connection which allows the GUI's read_to_string to finish.
    }

    fn process_command(&self, cmd: IpcCommand) -> IpcResponse {
        match cmd.command.as_str() {
            "register" => {
                let uploader = self.uploader.clone();
                let server_ip = cmd.payload["server_ip"].as_str().unwrap_or("").to_string();
                let server_port = cmd.payload["server_port"].as_str().unwrap_or("").to_string();
                let _cpe_id = cmd.payload["cpe_id"].as_str().unwrap_or("").to_string();
                let pin = cmd.payload["pin"].as_str().unwrap_or("").to_string();

                if server_ip.is_empty() || server_port.is_empty() {
                    return IpcResponse {
                        status: "error".to_string(),
                        message: "Missing server_ip or server_port".to_string(),
                        payload: None,
                    };
                }

                println!("Registering device via IPC: {}:{}", server_ip, server_port);

                let base_url = format!("http://{}:{}", server_ip, server_port);
                let app_code = "mac_monitor".to_string();
                let app_secret = pin.clone();
                let serial_number = "MAC_SN_123456".to_string(); // TODO: 获取真实序列号

                uploader.update_config(&app_code, &app_secret, &base_url, &serial_number);

                // 尝试保存到配置文件 (硬编码路径需注意，但在本项目中是一致的)
                let config_path = "/Users/adolf/Desktop/code/clash/mac-monitor-project/audit-service/config.json";
                if let Ok(config_str) = std::fs::read_to_string(config_path) {
                    if let Ok(mut json_val) = serde_json::from_str::<serde_json::Value>(&config_str) {
                        json_val["server"]["url"] = serde_json::Value::String(base_url);
                        json_val["server"]["app_code"] = serde_json::Value::String(app_code);
                        json_val["server"]["app_secret"] = serde_json::Value::String(app_secret);

                        if let Ok(new_config_str) = serde_json::to_string_pretty(&json_val) {
                            let _ = std::fs::write(config_path, new_config_str);
                        }
                    }
                }

                IpcResponse {
                    status: "ok".to_string(),
                    message: "Registration successful".to_string(),
                    payload: None,
                }
            }
            "login" => {
                let uploader = self.uploader.clone();
                let (tx, rx) = std::sync::mpsc::channel();

                println!("Processing login via IPC...");

                self.runtime_handle.spawn(async move {
                    // 调用 uploader 的 heartbeat 或类似逻辑来获取 token
                    // 这里我们假设 heartbeat 会触发 get_token
                    let result = uploader.heartbeat("0.1.0").await;
                    let _ = tx.send(result);
                });

                match rx.recv_timeout(std::time::Duration::from_secs(15)) {
                    Ok(Ok(resp)) => {
                        // 返回包含 token 的 payload
                        IpcResponse {
                            status: "ok".to_string(),
                            message: "Login successful".to_string(),
                            payload: Some(serde_json::json!({
                                "token": "mock_token_via_heartbeat" // 实际上 get_token 已经把 token 存入 uploader 了
                            })),
                        }
                    },
                    Ok(Err(e)) => IpcResponse { status: "error".to_string(), message: e, payload: None },
                    Err(_) => IpcResponse { status: "error".to_string(), message: "Login timeout".to_string(), payload: None },
                }
            }
            "logout" => {
                println!("Processing logout...");
                IpcResponse {
                    status: "ok".to_string(),
                    message: "Logged out".to_string(),
                    payload: None,
                }
            }
            "get_pops" => {
                let uploader = self.uploader.clone();
                let (tx, rx) = std::sync::mpsc::channel();

                self.runtime_handle.spawn(async move {
                    let result = uploader.get_pop_list().await;
                    let _ = tx.send(result);
                });

                match rx.recv_timeout(std::time::Duration::from_secs(10)) {
                    Ok(Ok(pops)) => IpcResponse {
                        status: "ok".to_string(),
                        message: "Success".to_string(),
                        payload: Some(serde_json::to_value(pops).unwrap()),
                    },
                    Ok(Err(e)) => IpcResponse { status: "error".to_string(), message: e, payload: None },
                    Err(_) => IpcResponse { status: "error".to_string(), message: "Timeout".to_string(), payload: None },
                }
            }
            "check_update" => {
                let uploader = self.uploader.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                self.runtime_handle.spawn(async move {
                    let _ = tx.send(uploader.check_update().await);
                });
                match rx.recv_timeout(std::time::Duration::from_secs(10)) {
                    Ok(Ok(info)) => IpcResponse {
                        status: "ok".to_string(),
                        message: "Success".to_string(),
                        payload: Some(serde_json::to_value(info).unwrap()),
                    },
                    Ok(Err(e)) => IpcResponse { status: "error".to_string(), message: e, payload: None },
                    Err(_) => IpcResponse { status: "error".to_string(), message: "Timeout".to_string(), payload: None },
                }
            }
            "get_cert" => {
                let uploader = self.uploader.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                self.runtime_handle.spawn(async move {
                    let _ = tx.send(uploader.get_cert_info().await);
                });
                match rx.recv_timeout(std::time::Duration::from_secs(10)) {
                    Ok(Ok(info)) => IpcResponse {
                        status: "ok".to_string(),
                        message: "Success".to_string(),
                        payload: Some(serde_json::to_value(info).unwrap()),
                    },
                    Ok(Err(e)) => IpcResponse { status: "error".to_string(), message: e, payload: None },
                    Err(_) => IpcResponse { status: "error".to_string(), message: "Timeout".to_string(), payload: None },
                }
            }
            "get_server_time" => {
                let uploader = self.uploader.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                self.runtime_handle.spawn(async move {
                    let _ = tx.send(uploader.get_server_time().await);
                });
                match rx.recv_timeout(std::time::Duration::from_secs(10)) {
                    Ok(Ok(time)) => IpcResponse {
                        status: "ok".to_string(),
                        message: "Success".to_string(),
                        payload: Some(serde_json::to_value(time).unwrap()),
                    },
                    Ok(Err(e)) => IpcResponse { status: "error".to_string(), message: e, payload: None },
                    Err(_) => IpcResponse { status: "error".to_string(), message: "Timeout".to_string(), payload: None },
                }
            }
            "log_traffic" => {
                match serde_json::from_value::<AuditLog>(cmd.payload) {
                    Ok(log) => {
                        let db = self.db.clone();
                        self.runtime_handle.spawn(async move {
                            if let Err(e) = db.save_audit_log(&log).await {
                                eprintln!("Failed to save audit log via IPC: {}", e);
                            }
                        });

                        IpcResponse {
                            status: "ok".to_string(),
                            message: "Log queued".to_string(),
                            payload: None,
                        }
                    }
                    Err(e) => IpcResponse {
                        status: "error".to_string(),
                        message: format!("Invalid AuditLog payload: {}", e),
                        payload: None,
                    }
                }
            }
            _ => IpcResponse {
                status: "error".to_string(),
                message: format!("Unknown command: {}", cmd.command),
                payload: None,
            },
        }
    }
}
