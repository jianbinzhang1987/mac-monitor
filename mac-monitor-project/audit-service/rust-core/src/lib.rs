pub mod models;
pub mod db;
pub mod uploader;
pub mod clock;
pub mod ipc;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;
use tokio::runtime::Runtime;
use image::{ImageBuffer, Rgba, DynamicImage};
use image::codecs::jpeg::JpegEncoder;
use sha2::{Sha256, Digest};
use std::io::Cursor;
use chrono::Local;

use crate::db::Database;
use crate::models::{AuditLog, ScreenshotLog};
use crate::uploader::Uploader;
use crate::uploader::sync::SyncService;
use crate::clock::LogicalClock;
use crate::ipc::IpcServer;

lazy_static::lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().expect("Failed to create Tokio runtime");
    static ref SERVICE_CONTEXT: Arc<ServiceContext> = setup_context();
}

struct ServiceContext {
    db: Arc<Database>,
    uploader: Arc<Uploader>,
    clock: Arc<LogicalClock>,
}

fn setup_context() -> Arc<ServiceContext> {
    RUNTIME.block_on(async {
        let db = Database::new("sqlite:audit.db").await.expect("Failed to init DB");
        let uploader = Arc::new(Uploader::new("APP_CODE", "APP_SECRET", "https://api.example.com"));
        let clock = Arc::new(LogicalClock::new());

        let db_arc = Arc::new(db);

        // 初始化背景同步服务
        let sync_service = SyncService::new(db_arc.clone(), uploader.clone(), clock.clone());
        sync_service.start();

        // 启动 IPC 服务
        let ipc_server = IpcServer::new(db_arc.clone(), uploader.clone(), RUNTIME.handle().clone());
        ipc_server.start();

        Arc::new(ServiceContext {
            db: db_arc,
            uploader,
            clock,
        })
    })
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
    if ptr.is_null() || len == 0 {
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
        let ctx = &SERVICE_CONTEXT;

        // 1. 构建 ImageBuffer (Swift 传过来的是 RGBA)
        let img: Option<ImageBuffer<Rgba<u8>, _>> = ImageBuffer::from_raw(width, height, data_vec);

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
                log::error!("Failed to encode image to JPEG: {}", e);
                return;
            }

            // 5. 保存图片到本地
            let filename = format!("{}.jpg", hash_string);
            let save_path = format!("audit_images/{}", filename);
            let _ = std::fs::create_dir_all("audit_images");

            if let Err(e) = std::fs::write(&save_path, &jpeg_data) {
                log::error!("Failed to save image file: {}", e);
                return;
            }

            // 6. 创建日志记录
            let log = ScreenshotLog {
                id: None,
                pin: "user_pin".to_string(), // TODO: 获取真实 PIN
                capture_time: Local::now().to_rfc3339(),
                app_name: app_name_str,
                window_title: "Active Window".to_string(), // 未来可以传递更多窗口细节
                image_path: save_path,
                image_hash: hash_string,
                is_sensitive,
                ocr_text: ocr_text_str,
                host_id: "host_123".to_string(),
                cpe_id: "cpe_123".to_string(),
                mac: "00:00:00:00:00:00".to_string(),
                ip: "127.0.0.1".to_string(),
            };

            if let Err(e) = ctx.db.save_screenshot_log(&log).await {
                log::error!("Failed to save screenshot log to DB: {}", e);
            } else {
                log::info!("📸 Screenshot saved: {} (Sensitive: {})", log.app_name, is_sensitive);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn log_audit_event(event_json: *const c_char) {
    if event_json.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(event_json) };
    let r_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return,
    };

    RUNTIME.spawn(async move {
        let ctx = &SERVICE_CONTEXT;
        
        // 1. 先解析为通用的 Value 以判断类型
        let v: serde_json::Value = match serde_json::from_str(&r_str) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Failed to parse log JSON Value: {}", e);
                return;
            }
        };

        let log_type = v["type"].as_str().unwrap_or("unknown");

        match log_type {
            "behavior" => {
                match serde_json::from_str::<BehaviorLog>(&r_str) {
                    Ok(log) => {
                        if let Err(e) = ctx.db.save_behavior_log(&log).await {
                            log::error!("Failed to save behavior log: {}", e);
                        } else {
                            log::info!("🛡 Behavior log saved: {} - {}", log.op_type, log.op_reason);
                        }
                    }
                    Err(e) => log::error!("Failed to parse BehaviorLog: {}", e),
                }
            }
            _ => {
                // 默认为审计日志 (exec, write, flow 等)
                match serde_json::from_str::<AuditLog>(&r_str) {
                    Ok(log) => {
                        if let Err(e) = ctx.db.save_audit_log(&log).await {
                            log::error!("Failed to save audit log: {}", e);
                        } else {
                            log::info!("Audit log saved: {}", log.id);
                        }
                    }
                    Err(e) => {
                        // 如果还是失败，尝试作为简单的 BehaviorLog 解析 (兜底)
                        log::error!("Failed to parse AuditLog: {}. Payload: {}", e, r_str);
                    }
                }
            }
        }
    });
}
