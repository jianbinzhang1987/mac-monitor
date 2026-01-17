// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, State,
};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;
use tokio::time::{self, Duration};
use interprocess::local_socket::LocalSocketStream;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppState {
    is_logged_in: bool,
    pin: String,
    token: String,
    server_ip: String,
    server_port: String,
    cpe_id: String,
    device_info: Option<DeviceInfoPayload>,
    current_pop: Option<PopNode>,
    available_pops: Vec<PopNode>,
    audit_policy_json: Option<String>,
    logic_clock: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PopNode {
    id: String,
    name: String,
    latency: u32,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DeviceInfoPayload {
    pin_number: String,
    ip: String,
    mac: String,
    cpe_id: String,
    host_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AuditPolicyPayload {
    policy_json: String,
}

async fn send_ipc_command(command: &str, payload: serde_json::Value) -> Result<String, String> {
    let socket_path = "/tmp/mac_monitor_audit.sock";
    println!("📡 Sending IPC command: {} to {}", command, socket_path);

    // 检查套接字文件是否存在
    if !std::path::Path::new(socket_path).exists() {
        let err = "审计服务未启动（套接字文件不存在）".to_string();
        eprintln!("❌ IPC Error: {}", err);
        return Err(err);
    }

    let mut stream = LocalSocketStream::connect(socket_path)
        .map_err(|e| {
            let err = format!("无法连接到审计服务: {}", e);
            eprintln!("❌ IPC Connection Error: {}", err);
            err
        })?;

    let full_command = serde_json::json!({
        "command": command,
        "payload": payload
    });

    let cmd_str = full_command.to_string();
    println!("📤 Writing to socket: {}", cmd_str);
    stream.write_all(cmd_str.as_bytes())
        .map_err(|e| {
            let err = format!("发送指令失败: {}", e);
            eprintln!("❌ IPC Write Error: {}", err);
            err
        })?;

    stream.flush().map_err(|e| e.to_string())?;

    let mut response = String::new();
    println!("📥 Waiting for IPC response...");
    stream.read_to_string(&mut response)
        .map_err(|e| {
            let err = format!("读取响应失败: {}", e);
            eprintln!("❌ IPC Read Error: {}", err);
            err
        })?;

    if response.is_empty() {
        let err = "审计服务响应为空".to_string();
        eprintln!("❌ IPC Empty Response");
        return Err(err);
    }

    println!("✅ Received IPC response: {}", response);
    Ok(response)
}

fn device_info_path(app_handle: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let base_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("无法获取配置目录: {}", e))?;
    std::fs::create_dir_all(&base_dir).map_err(|e| format!("无法创建配置目录: {}", e))?;
    Ok(base_dir.join("device_info.json"))
}

#[derive(Debug, Deserialize)]
struct IpcResponse {
    status: String,
    message: String,
    payload: Option<serde_json::Value>,
}

#[tauri::command]
async fn get_pop_nodes(state: State<'_, ManagedState>) -> Result<Vec<PopNode>, String> {
    let res_str = send_ipc_command("get_pops", serde_json::Value::Null).await?;
    let res: IpcResponse = serde_json::from_str(&res_str).map_err(|e| e.to_string())?;
    
    if res.status == "ok" {
        if let Some(payload) = res.payload {
            let raw_pops: Vec<serde_json::Value> = serde_json::from_value(payload).map_err(|e| e.to_string())?;
            let mut pops = Vec::new();
            for p in raw_pops {
                pops.push(PopNode {
                    id: p["pop_id"].as_str().unwrap_or("").to_string(),
                    name: p["name"].as_str().unwrap_or("Unknown").to_string(),
                    latency: p["latency_hint"].as_u64().unwrap_or(0) as u32,
                });
            }
            let mut s = state.0.lock().unwrap();
            s.available_pops = pops.clone();
            Ok(pops)
        } else {
            Ok(vec![])
        }
    } else {
        Err(res.message)
    }
}

#[tauri::command]
async fn switch_pop_node(node_id: String, state: State<'_, ManagedState>) -> Result<String, String> {
    let node = {
        let s = state.0.lock().unwrap();
        s.available_pops.iter().find(|n| n.id == node_id).cloned()
    };

    if let Some(node) = node {
        let mut s = state.0.lock().unwrap();
        s.current_pop = Some(node.clone());
        println!("Switching to POP node: {}", node.name);
        Ok(format!("已切换至 {}", node.name))
    } else {
        Err("找不到指定的节点".to_string())
    }
}

#[tauri::command]
async fn check_for_updates() -> Result<serde_json::Value, String> {
    let res_str = send_ipc_command("check_update", serde_json::Value::Null).await?;
    let res: IpcResponse = serde_json::from_str(&res_str).map_err(|e| e.to_string())?;
    
    if res.status == "ok" {
        Ok(res.payload.unwrap_or(serde_json::json!({ "has_update": false })))
    } else {
        Err(res.message)
    }
}

#[tauri::command]
async fn register(payload: RegisterPayload, state: State<'_, ManagedState>) -> Result<String, String> {
    println!("Registering device: {:?}", payload);

    let _ipc_res = send_ipc_command("register", serde_json::to_value(&payload).unwrap()).await?;
    // println!("Audit service response: {}", ipc_res); // Removed as per new code

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
    let res: IpcResponse = serde_json::from_str(&ipc_res).map_err(|e| e.to_string())?;

    let mut s = state.0.lock().unwrap();
    s.pin = payload.pin;
    s.is_logged_in = true;
    
    if let Some(payload) = res.payload {
        if let Some(token) = payload["token"].as_str() {
            s.token = token.to_string();
        }
    }

    Ok("登录成功".to_string())
}

#[tauri::command]
async fn set_device_info(
    payload: DeviceInfoPayload,
    // app_handle: tauri::AppHandle, // Removed as per new code
    state: State<'_, ManagedState>,
) -> Result<String, String> {
    // let path = device_info_path(&app_handle)?; // Removed as per new code
    // let data = serde_json::to_vec(&payload).map_err(|e| format!("序列化失败: {}", e))?; // Removed as per new code
    // std::fs::write(&path, data).map_err(|e| format!("写入配置失败: {}", e))?; // Removed as per new code

    let mut s = state.0.lock().unwrap();
    s.device_info = Some(payload);

    Ok("设备信息已同步".to_string())
}

#[tauri::command]
async fn set_audit_policy(
    payload: AuditPolicyPayload,
    app_handle: tauri::AppHandle,
    state: State<'_, ManagedState>,
) -> Result<String, String> {
    let base_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("无法获取配置目录: {}", e))?;
    std::fs::create_dir_all(&base_dir).map_err(|e| format!("无法创建配置目录: {}", e))?;
    let path = base_dir.join("audit_policy.json");
    std::fs::write(&path, payload.policy_json.as_bytes())
        .map_err(|e| format!("写入策略失败: {}", e))?;

    let mut s = state.0.lock().unwrap();
    s.audit_policy_json = Some(payload.policy_json);

    Ok("审计策略已保存".to_string())
}

#[tauri::command]
async fn start_vpn(app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("🔌 Starting VPN via sidecar...");
    let sidecar = app_handle.shell().sidecar("vpn-helper")
        .map_err(|e| format!("找不到 vpn-helper: {}", e))?;
    
    let (_rx, _child) = sidecar.arg("--start").spawn()
        .map_err(|e| format!("启动 vpn-helper 失败: {}", e))?;
    
    Ok("VPN 启动指令已发送".to_string())
}

#[tauri::command]
async fn stop_vpn(app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("🛑 Stopping VPN via sidecar...");
    let sidecar = app_handle.shell().sidecar("vpn-helper")
        .map_err(|e| format!("找不到 vpn-helper: {}", e))?;
    
    let (_rx, _child) = sidecar.arg("--stop").spawn()
        .map_err(|e| format!("停止 vpn-helper 失败: {}", e))?;
    
    Ok("VPN 停止指令已发送".to_string())
}

#[tauri::command]
async fn get_vpn_status(app_handle: tauri::AppHandle) -> Result<i32, String> {
    let sidecar = app_handle.shell().sidecar("vpn-helper")
        .map_err(|e| format!("找不到 vpn-helper: {}", e))?;
    
    let output = sidecar.arg("--status").output().await
        .map_err(|e| format!("获取 VPN 状态失败: {}", e))?;
    
    if output.status.success() {
        let out_str = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = out_str.lines().find(|l| l.contains("VPN Status:")) {
            if let Some(status_str) = line.split(":").last() {
                return status_str.trim().parse::<i32>().map_err(|e| e.to_string());
            }
        }
    }
    Err("无法解析 VPN 状态".to_string())
}

fn start_heartbeat_loop(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));
        let client = reqwest::Client::new();
        
        loop {
            interval.tick().await;
            let (is_logged, token, server_ip, server_port, pop_id, clock) = {
                let state = app_handle.state::<ManagedState>();
                let mut s = state.0.lock().unwrap();
                s.logic_clock += 1; // 递增本地逻辑时钟
                (
                    s.is_logged_in, 
                    s.token.clone(), 
                    s.server_ip.clone(), 
                    s.server_port.clone(),
                    s.current_pop.as_ref().map(|p| p.id.clone()),
                    s.logic_clock
                )
            };

            if is_logged && !server_ip.is_empty() {
                let url = format!("http://{}:{}/api/v1/heartbeat", server_ip, server_port);
                let payload = serde_json::json!({
                    "token": token,
                    "logic_clock": clock,
                    "pop_id": pop_id,
                    "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
                });

                println!("💓 Heartbeat: sending to {}", url);
                
                match client.post(&url).json(&payload).send().await {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            if let Ok(data) = resp.json::<serde_json::Value>().await {
                                // 处理服务端逻辑时钟校准
                                if let Some(srv_clock) = data["server_logic_clock"].as_u64() {
                                    let state = app_handle.state::<ManagedState>();
                                    let mut s = state.0.lock().unwrap();
                                    if srv_clock > s.logic_clock {
                                        s.logic_clock = srv_clock;
                                    }
                                }
                                println!("✅ Heartbeat success: server sync clock");
                            }
                        } else {
                            eprintln!("❌ Heartbeat failed with status: {}", resp.status());
                        }
                    }
                    Err(e) => eprintln!("❌ Heartbeat network error: {}", e),
                }
            }
        }
    });
}

// Helper to run vpn-helper with sudo via AppleScript
fn run_vpn_helper_auth(arg: &str, handle: tauri::AppHandle) -> Result<String, String> {
    let bundle_path = handle.path().resource_dir().map_err(|e| e.to_string())?;
    // Expected path in production bundle: Contents/Resources/bin/vpn-helper-x86_64-apple-darwin
    // Or externalBin dir. 
    // Sidecar means binaries are often side-by-side with main binary in MacOS/ or in Resources/bin
    
    // For simplicity in this iteration, we try to find it relative to current execution
    // But since we are calling it with sudo via script, absolute path is safest.
    
    // Let's resolve the path to the sidecar "vpn-helper"
    let sidecar_path = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .unwrap()
        .join("vpn-helper"); // The sidecar tool name (without arch suffix usually if renamed by tauri, but we manually copied it)

    // Actually, we should just assume it's where we put it.
    // In dev: gui-app/src-tauri/bin/vpn-helper...
    // In prod: Contents/MacOS/vpn-helper
    
    // Let's rely on finding it next to the app binary which is robust enough for now
    let helper_path = sidecar_path.to_string_lossy().to_string();
    
    // The command to run
    let script = format!(
        "do shell script \"'{}' {}\" with administrator privileges",
        helper_path, arg
    );

    println!("🔑 Requesting privilege for: {}", script);

    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
async fn enable_proxy(app_handle: tauri::AppHandle) -> Result<String, String> {
    // Invoke vpn-helper --enable-proxy with admin privs
    run_vpn_helper_auth("--enable-proxy", app_handle)
}

#[tauri::command]
async fn disable_proxy(app_handle: tauri::AppHandle) -> Result<String, String> {
    run_vpn_helper_auth("--disable-proxy", app_handle)
}

#[tauri::command]
async fn enable_monitoring(app_handle: tauri::AppHandle) -> Result<String, String> {
    run_vpn_helper_auth("--enable-monitoring", app_handle)
}

#[tauri::command]
async fn disable_monitoring(app_handle: tauri::AppHandle) -> Result<String, String> {
    run_vpn_helper_auth("--disable-monitoring", app_handle)
}

#[tauri::command]
async fn set_redaction_status(enabled: bool) -> Result<String, String> {
    let res_str = send_ipc_command("set_redaction_status", serde_json::json!({ "enabled": enabled })).await?;
    let res: IpcResponse = serde_json::from_str(&res_str).map_err(|e| e.to_string())?;

    if res.status == "ok" {
        Ok(res.message)
    } else {
        Err(res.message)
    }
}

#[tauri::command]
async fn get_screenshot_logs() -> Result<serde_json::Value, String> {
    let res_str = send_ipc_command("get_screenshot_logs", serde_json::Value::Null).await?;
    let res: IpcResponse = serde_json::from_str(&res_str).map_err(|e| e.to_string())?;

    if res.status == "ok" {
        Ok(res.payload.unwrap_or(serde_json::json!([])))
    } else {
        Err(res.message)
    }
}

fn main() {
    let app_state = ManagedState(Arc::new(Mutex::new(AppState {
        is_logged_in: false,
        pin: String::new(),
        server_ip: String::new(),
        server_port: String::new(),
        cpe_id: String::new(),
        device_info: None,
        current_pop: None,
        available_pops: vec![
            PopNode { id: "hk-01".into(), name: "香港 CN2 01".into(), latency: 25 },
            PopNode { id: "sg-01".into(), name: "新加坡 BGP 01".into(), latency: 45 },
            PopNode { id: "jp-01".into(), name: "东京 NTT 01".into(), latency: 60 },
        ],
        logic_clock: 0,
        token: String::new(),
        audit_policy_json: None,
    })));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![])))
        .manage(app_state)
        .setup(|app| {
            let handle = app.handle().clone();

            // Enable AutoSart (OS Login Item)
            let _ = app.autolaunch().enable();

            // Auto-start Monitoring on App Launch
            tauri::async_runtime::spawn(async move {
                time::sleep(Duration::from_secs(1)).await;
                println!("🚀 Auto-enabling Monitoring Service...");
                match enable_monitoring(handle.clone()).await {
                    Ok(out) => println!("✅ Monitoring Auto-Enabled: {}", out),
                    Err(e) => eprintln!("❌ Failed to Auto-Enable Monitoring: {}", e),
                }
            });

            let handle = app.handle().clone();
            // Auto-start Proxy on App Launch (User Request)
            // This will prompt for password if not already authorized recently
            tauri::async_runtime::spawn(async move {
                // Delay slightly to let UI show up first
                time::sleep(Duration::from_secs(3)).await;
                println!("🚀 Auto-enabling HTTP Proxy...");
                match enable_proxy(handle.clone()).await {
                    Ok(out) => println!("✅ Proxy Auto-Enabled: {}", out),
                    Err(e) => eprintln!("❌ Failed to Auto-Enable Proxy: {}", e),
                }
            });

            let handle = app.handle();
            // 1. 创建托盘图标及其菜单（Tauri 2 风格）
            let show_i = MenuItem::with_id(handle, "show", "显示主界面", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(handle, "quit", "退出客户端 (需验证)", true, None::<&str>)?;
            let menu = Menu::with_items(handle, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app: &tauri::AppHandle, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            start_heartbeat_loop(handle.clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register, login, set_device_info, set_audit_policy,
            get_pop_nodes, switch_pop_node, check_for_updates,
            start_vpn, stop_vpn, get_vpn_status,
            enable_proxy, disable_proxy,
            enable_monitoring, disable_monitoring,
            get_screenshot_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
