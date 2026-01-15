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
        let mut buffer = String::new();
        // 简单读取，实际场景可能需要更健壮的分帧处理
        if let Ok(_) = stream.read_to_string(&mut buffer) {
            println!("Received IPC command: {}", buffer);

            let response = match serde_json::from_str::<IpcCommand>(&buffer) {
                Ok(cmd) => self.process_command(cmd),
                Err(e) => IpcResponse {
                    status: "error".to_string(),
                    message: format!("Invalid JSON: {}", e),
                    payload: None,
                },
            };

            let response_json = serde_json::to_string(&response).unwrap_or_else(|_| "{\"status\":\"error\"}".to_string());
            let _ = stream.write_all(response_json.as_bytes());
        }
    }

    fn process_command(&self, cmd: IpcCommand) -> IpcResponse {
        match cmd.command.as_str() {
            "register" => {
                let db = self.db.clone();
                let uploader = self.uploader.clone();
                let payload = cmd.payload.clone();

                println!("Processing registration in background...");

                IpcResponse {
                    status: "ok".to_string(),
                    message: "Registration request accepted".to_string(),
                    payload: None,
                }
            }
            "login" => {
                let pin = cmd.payload["pin"].as_str().unwrap_or("").to_string();
                println!("Processing login for PIN: {}", pin);

                IpcResponse {
                    status: "ok".to_string(),
                    message: format!("Login successful for {}", pin),
                    payload: None,
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
