pub mod models;
pub mod db;
pub mod uploader;
pub mod clock;
pub mod ipc;
pub mod scanner;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use tokio::sync::OnceCell;
use image::{ImageBuffer, Rgba, DynamicImage};
use sha2::{Sha256, Digest};
use std::io::Cursor;
use chrono::Local;

use crate::db::Database;
use crate::models::{AuditLog, ScreenshotLog, BehaviorLog};
use crate::uploader::Uploader;
use crate::uploader::sync::SyncService;
use crate::clock::LogicalClock;
use crate::ipc::IpcServer;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    server: ServerConfig,
    storage: StorageConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    url: String,
    app_code: String,
    app_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct StorageConfig {
    screenshot_dir: String,
    database_path: String,
}

lazy_static::lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().expect("Failed to create Tokio runtime");
}

static SERVICE_CONTEXT: OnceCell<Arc<ServiceContext>> = OnceCell::const_new();

struct ServiceContext {
    db: Arc<Database>,
    uploader: Arc<Uploader>,
    clock: Arc<LogicalClock>,
    config: Config,
    policy: Arc<RwLock<models::PolicyConfig>>,
    device_info: models::DeviceInfo,
}

async fn init_service_context() -> Arc<ServiceContext> {
    // 1. 加载配置
    let config_path = "/Users/adolf/Desktop/code/clash/mac-monitor-project/audit-service/config.json";
    let config_str = fs::read_to_string(config_path).expect("Failed to read config.json");
    let config: Config = serde_json::from_str(&config_str).expect("Failed to parse config.json");

    // 2. 初始化数据库
    let _ = std::fs::create_dir_all(std::path::Path::new(&config.storage.database_path).parent().unwrap());
    let db = Database::new(&format!("sqlite://{}", config.storage.database_path))
        .await
        .expect("Failed to init DB");

    // 3. 获取真实设备信息
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    let host_name = sysinfo::System::host_name().unwrap_or_else(|| "Unknown-Mac".to_string());
    let mac_addr = get_local_mac();
    let ip_addr = get_local_ip();
    let serial_number = get_macos_serial_number();

    let device_info = models::DeviceInfo {
        pin: serial_number.clone(),
        host_id: host_name.clone(),
        cpe_id: serial_number.clone(), // 使用 clone 避免所有权转移
        mac: mac_addr,
        ip: ip_addr,
    };

    // 4. 初始化上传器
    let uploader = Arc::new(Uploader::new(
        &config.server.app_code,
        &config.server.app_secret,
        &config.server.url,
        &serial_number
    ));

    let clock = Arc::new(LogicalClock::new());
    let db_arc = Arc::new(db);

    // 初始化默认策略
    let policy = Arc::new(RwLock::new(models::PolicyConfig {
        process_blacklist: vec!["clash".to_string(), "v2ray".to_string(), "clash-meta".to_string(), "proxyman".to_string()],
        app_blacklist: vec!["clash".to_string(), "v2ray".to_string(), "proxyman".to_string()],
    }));

    // 5. 初始化背景同步服务
    let sync_service = SyncService::new(
        db_arc.clone(),
        uploader.clone(),
        clock.clone(),
        policy.clone(),
        device_info.clone(),
        config.storage.screenshot_dir.clone(),
    );
    sync_service.start();

    // 6. 启动 IPC 服务
    let ipc_server = IpcServer::new(db_arc.clone(), uploader.clone(), RUNTIME.handle().clone());
    ipc_server.start();

    Arc::new(ServiceContext {
        db: db_arc,
        uploader,
        clock,
        config,
        policy,
        device_info,
    })
}

fn get_macos_serial_number() -> String {
    use std::process::Command;
    let output = Command::new("ioreg")
        .args(&["-c", "IOPlatformExpertDevice", "-d", "2"])
        .output();

    match output {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout);
            for line in s.lines() {
                if line.contains("IOPlatformSerialNumber") {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() == 2 {
                        return parts[1].trim().trim_matches(|c| c == '"' || c == ' ').to_string();
                    }
                }
            }
        },
        Err(e) => log::error!("Failed to execute ioreg: {}", e),
    }
    "UNKNOWN_SERIAL".to_string()
}

fn get_primary_interface_name() -> Option<String> {
    use std::process::Command;
    let output = Command::new("route")
        .args(&["-n", "get", "default"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let s = String::from_utf8_lossy(&output.stdout);
    for line in s.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("interface:") {
            let iface = rest.trim();
            if !iface.is_empty() {
                log::info!("Primary network interface detected: {}", iface);
                return Some(iface.to_string());
            }
        }
    }
    None
}

fn get_ip_for_interface(interface: &str) -> Option<String> {
    use std::process::Command;
    let output = Command::new("ipconfig")
        .args(&["getifaddr", interface])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if ip.is_empty() {
        None
    } else {
        Some(ip)
    }
}

fn get_mac_for_interface(interface: &str) -> Option<String> {
    use std::process::Command;
    let output = Command::new("ifconfig")
        .arg(interface)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let s = String::from_utf8_lossy(&output.stdout);
    for line in s.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("ether ") {
            let mac = rest.split_whitespace().next().unwrap_or("").trim();
            if !mac.is_empty() {
                return Some(mac.to_string());
            }
        }
    }
    None
}

fn get_local_ip() -> String {
    if let Some(interface) = get_primary_interface_name() {
        if let Some(ip) = get_ip_for_interface(&interface) {
            log::info!("Local IP resolved via {}: {}", interface, ip);
            return ip;
        }
    }

    // Fallback: derive local IP from a UDP socket without sending packets.
    use std::net::UdpSocket;
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            if socket.connect("8.8.8.8:80").is_ok() {
                if let Ok(addr) = socket.local_addr() {
                    log::info!("Local IP resolved via UDP fallback: {}", addr.ip());
                    return addr.ip().to_string();
                }
            }
        }
        Err(e) => log::error!("Failed to bind UDP socket for IP detection: {}", e),
    }
    log::warn!("Local IP fallback to 127.0.0.1");
    "127.0.0.1".to_string()
}

fn get_local_mac() -> String {
    if let Some(interface) = get_primary_interface_name() {
        if let Some(mac) = get_mac_for_interface(&interface) {
            log::info!("Local MAC resolved via {}: {}", interface, mac);
            return mac;
        }
    }

    mac_address::get_mac_address()
        .unwrap_or(None)
        .map(|m| {
            let mac = m.to_string();
            log::info!("Local MAC resolved via mac_address: {}", mac);
            mac
        })
        .unwrap_or_else(|| {
            log::warn!("Local MAC fallback to 00:00:00:00:00:00");
            "00:00:00:00:00:00".to_string()
        })
}

async fn get_service_context() -> Arc<ServiceContext> {
    SERVICE_CONTEXT
        .get_or_init(|| async { init_service_context().await })
        .await
        .clone()
}

#[no_mangle]
pub extern "C" fn init_audit_core() {
    RUNTIME.spawn(async {
        let _ = get_service_context().await;
        log::info!("Audit Logic Core initialized successfully");
    });
}

#[no_mangle]
pub extern "C" fn analyze_enhanced_image(
    ptr: *const u8,
    len: usize,
    width: u32,
    height: u32,
    app_name: *const c_char,
    is_sensitive: bool,
    ocr_text: *const c_char
) {
    eprintln!(
        "analyze_enhanced_image called: len={}, width={}, height={}",
        len,
        width,
        height
    );
    if ptr.is_null() || len == 0 {
        eprintln!("analyze_enhanced_image: null ptr or zero len");
        return;
    }

    // 转换 FFI 字符串
    let app_name_str = if !app_name.is_null() {
        unsafe { CStr::from_ptr(app_name).to_string_lossy().into_owned() }
    } else {
        "Unknown".to_string()
    };

    let ocr_text_str = if !ocr_text.is_null() {
        Some(unsafe { CStr::from_ptr(ocr_text).to_string_lossy().into_owned() })
    } else {
        None
    };

    let raw_data = unsafe { std::slice::from_raw_parts(ptr, len) };
    let data_vec = raw_data.to_vec();

    RUNTIME.spawn(async move {
        let ctx = get_service_context().await;

        // 1. 构建 ImageBuffer (Swift 传过来的是 RGBA)
        let width_usize = width as usize;
        let height_usize = height as usize;
        let expected_row_bytes = width_usize.saturating_mul(4);
        if expected_row_bytes == 0 || height_usize == 0 {
            log::error!("Invalid image size: {}x{}", width, height);
            return;
        }
        if data_vec.len() < expected_row_bytes.saturating_mul(height_usize) {
            log::error!(
                "Buffer too small: len={}, expected at least {}",
                data_vec.len(),
                expected_row_bytes * height_usize
            );
            return;
        }
        if data_vec.len() % height_usize != 0 {
            log::error!(
                "Unexpected buffer size: len={}, height={}",
                data_vec.len(),
                height_usize
            );
            return;
        }
        let bytes_per_row = data_vec.len() / height_usize;
        if bytes_per_row < expected_row_bytes {
            log::error!(
                "Row stride too small: bytes_per_row={}, expected_row_bytes={}",
                bytes_per_row,
                expected_row_bytes
            );
            return;
        }
        let packed_data = if bytes_per_row == expected_row_bytes {
            data_vec
        } else {
            let mut packed = vec![0u8; expected_row_bytes * height_usize];
            for row in 0..height_usize {
                let src_start = row * bytes_per_row;
                let dst_start = row * expected_row_bytes;
                packed[dst_start..dst_start + expected_row_bytes]
                    .copy_from_slice(&data_vec[src_start..src_start + expected_row_bytes]);
            }
            packed
        };
        let img: Option<ImageBuffer<Rgba<u8>, _>> =
            ImageBuffer::from_raw(width, height, packed_data);

        if let Some(image_buffer) = img {
            let dynamic_image = DynamicImage::ImageRgba8(image_buffer);

            // 2. 计算图片哈希 (用于去重)
            let mut hasher = Sha256::new();
            hasher.update(dynamic_image.as_bytes());
            let hash_result = hasher.finalize();
            let hash_string = hex::encode(hash_result);

            // 3. 检查数据库是否存在相同哈希
            if let Ok(exists) = ctx.db.check_screenshot_exists(&hash_string).await {
                if exists {
                    // 如果存在，我们仍然可以更新 OCR 文本或日志，但为了性能通常选择跳过
                    log::info!("Duplicate screenshot detected, skipping save. Hash: {}", hash_string);
                    return;
                }
            }

            // 4. 将图片编码为 JPEG (压缩以减小体积)
            let mut jpeg_data = Vec::new();
            let mut cursor = Cursor::new(&mut jpeg_data);
            // 设置 80% 质量
            if let Err(e) = dynamic_image.write_to(&mut cursor, image::ImageFormat::Jpeg) {
                eprintln!("Failed to encode image to JPEG: {}", e);
                return;
            }

            // 5. 保存图片到本地
            let filename = format!("{}.jpg", hash_string);
            let save_dir = ctx.config.storage.screenshot_dir.as_str();
            let save_path = format!("{}/{}", save_dir, filename);
            if let Err(e) = std::fs::create_dir_all(save_dir) {
                eprintln!("Failed to create screenshot dir {}: {}", save_dir, e);
                return;
            }

            if let Err(e) = std::fs::write(&save_path, &jpeg_data) {
                eprintln!("Failed to save image file: {}", e);
                return;
            } else {
                eprintln!("Screenshot written to {}", save_path);
            }

            // 6. 创建日志记录
            let log = ScreenshotLog {
                id: None,
                capture_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                app_name: app_name_str,
                image_path: save_path.clone(),
                image_hash: hash_string,
                risk_level: if is_sensitive { 1 } else { 0 },
                ocr_text: ocr_text_str,
                host_id: ctx.device_info.host_id.clone(),
                cpe_id: ctx.device_info.cpe_id.clone(),
                mac: ctx.device_info.mac.clone(),
                ip: ctx.device_info.ip.clone(),
            };

            if let Err(e) = ctx.db.save_screenshot_log(&log).await {
                eprintln!("Failed to save screenshot log to DB: {}", e);
            } else {
                eprintln!("📸 Screenshot saved: {} (Sensitive: {})", log.app_name, is_sensitive);
            }
        } else {
            eprintln!("Failed to build ImageBuffer from raw pixels");
        }
    });
}


#[no_mangle]
pub extern "C" fn register_device(
    server_ip: *const c_char,
    server_port: *const c_char,
    cpe_id: *const c_char,
    pin: *const c_char
) -> bool {
    // 1. Convert C strings to Rust strings
    let server_ip = if !server_ip.is_null() {
        unsafe { CStr::from_ptr(server_ip).to_string_lossy().into_owned() }
    } else { return false; };

    let server_port = if !server_port.is_null() {
        unsafe { CStr::from_ptr(server_port).to_string_lossy().into_owned() }
    } else { return false; };

    let cpe_id = if !cpe_id.is_null() {
        unsafe { CStr::from_ptr(cpe_id).to_string_lossy().into_owned() }
    } else { return false; };

    let pin = if !pin.is_null() {
        unsafe { CStr::from_ptr(pin).to_string_lossy().into_owned() }
    } else { return false; };

    log::info!("Registering device: IP={}, Port={}, CPE={}, PIN={}", server_ip, server_port, cpe_id, pin);

    // 2. Construct Base URL
    let base_url = format!("http://{}:{}", server_ip, server_port);
    let app_code = "mac_monitor".to_string(); // Default app code
    let app_secret = pin.clone(); // Use PIN as secret for now

    // 3. Update ServiceContext
    let rt = &RUNTIME;
    let res = rt.block_on(async {
        let ctx = get_service_context().await;

        // Update Uploader config
        ctx.uploader.update_config(&app_code, &app_secret, &base_url, &ctx.device_info.pin);

        // Persist to config.json
        let config_path = "/Users/adolf/Desktop/code/clash/mac-monitor-project/audit-service/config.json";

        // Create new config object
        let new_config = Config {
            server: ServerConfig {
                url: base_url.clone(),
                app_code: app_code.clone(),
                app_secret: app_secret.clone(),
            },
            storage: StorageConfig {
                screenshot_dir: "/Users/adolf/Desktop/mac-monitor/screenshots".to_string(), // Keep default or read from existing
                database_path: ctx.config.storage.database_path.clone(),
            }
        };

        match serde_json::to_string_pretty(&new_config) {
            Ok(json) => {
                if let Err(e) = fs::write(config_path, json) {
                    log::error!("Failed to write config.json: {}", e);
                    return false;
                }
            }
            Err(e) => {
                log::error!("Failed to serialize config: {}", e);
                return false;
            }
        }

        true
    });

    res
}
