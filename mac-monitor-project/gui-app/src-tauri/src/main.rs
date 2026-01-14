// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};
use tokio::time::{self, Duration};
use interprocess::local_socket::LocalSocketStream;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppState {
    is_logged_in: bool,
    pin: String,
    server_ip: String,
    server_port: String,
    cpe_id: String,
}

struct ManagedState(Arc<Mutex<AppState>>);

#[derive(Debug, Serialize, Deserialize)]
struct RegisterPayload {
    server_ip: String,
    server_port: String,
    cpe_id: String,
    pin: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginPayload {
    pin: String,
}

async fn send_ipc_command(command: &str, payload: serde_json::Value) -> Result<String, String> {
    let socket_path = "/tmp/mac_monitor_audit.sock";
    let mut stream = LocalSocketStream::connect(socket_path)
        .map_err(|e| format!("无法连接到审计服务: {}", e))?;

    let full_command = serde_json::json!({
        "command": command,
        "payload": payload
    });

    let cmd_str = full_command.to_string();
    stream.write_all(cmd_str.as_bytes())
        .map_err(|e| format!("发送指令失败: {}", e))?;

    // Shutdown write half if supported or just wait for response
    // For simplicity, we expect a short JSON response
    let mut response = String::new();
    stream.read_to_string(&mut response)
        .map_err(|e| format!("读取响应失败: {}", e))?;

    Ok(response)
}

#[tauri::command]
async fn register(payload: RegisterPayload, state: State<'_, ManagedState>) -> Result<String, String> {
    println!("Registering device: {:?}", payload);

    let ipc_res = send_ipc_command("register", serde_json::to_value(&payload).unwrap()).await?;
    println!("Audit service response: {}", ipc_res);

    let mut s = state.0.lock().unwrap();
    s.server_ip = payload.server_ip;
    s.server_port = payload.server_port;
    s.cpe_id = payload.cpe_id;
    s.pin = payload.pin.clone();

    Ok("注册成功".to_string())
}

#[tauri::command]
async fn login(payload: LoginPayload, state: State<'_, ManagedState>) -> Result<String, String> {
    println!("Logging in with PIN: {}", payload.pin);

    let ipc_res = send_ipc_command("login", serde_json::to_value(&payload).unwrap()).await?;
    println!("Audit service response: {}", ipc_res);

    let mut s = state.0.lock().unwrap();
    s.pin = payload.pin;
    s.is_logged_in = true;

    Ok("登录成功".to_string())
}

fn start_heartbeat_loop(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            let state = app_handle.state::<ManagedState>();
            let s = state.0.lock().unwrap().clone();

            if s.is_logged_in {
                println!("Sending heartbeat for PIN: {}", s.pin);
                // TODO: Perform HTTP POST to heartbeat endpoint
                // let client = reqwest::Client::new();
                // ...
            }
        }
    });
}

fn main() {
    let app_state = ManagedState(Arc::new(Mutex::new(AppState {
        is_logged_in: false,
        pin: String::new(),
        server_ip: String::new(),
        server_port: String::new(),
        cpe_id: String::new(),
    })));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .setup(|app| {
            start_heartbeat_loop(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![register, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
