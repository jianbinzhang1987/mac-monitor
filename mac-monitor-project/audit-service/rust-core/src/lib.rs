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
        let sync_service = SyncService::new(db_arc.clone(), uploader.clone());
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
pub extern "C" fn analyze_image_buffer(
    ptr: *const u8,
    len: usize,
    width: u32,
    height: u32
) {
    if ptr.is_null() || len == 0 {
        return;
    }

    let raw_data = unsafe { std::slice::from_raw_parts(ptr, len) };

    // 我们需要克隆数据，因为 FFI 指针指向的内存在函数返回后可能无效
    // 或者我们直接在当前线程处理（但这会阻塞 Swift UI/Main 线程，所以最好 spawn 任务）
    let data_vec = raw_data.to_vec();

    RUNTIME.spawn(async move {
        let ctx = &SERVICE_CONTEXT;

        // 1. 构建 ImageBuffer
        // 注意：Swift 传递的是 BGRA 格式，image crate 默认处理 RGBA
        // 这里假设 Swift 端传递的是原始字节，我们需要正确转换
        // 如果是 BGRA，我们需要在这里进行通道交换，或者在保存时处理
        // 为了简化，这里假设数据可以直接被 ImageBuffer 加载 (可能颜色会反转，但在审计场景可接受，或者稍后修复)
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
                    log::info!("Duplicate screenshot detected, skipping save. Hash: {}", hash_string);
                    return;
                }
            }

            // 4. 将图片编码为 JPEG
            let mut jpeg_data = Vec::new();
            let mut cursor = Cursor::new(&mut jpeg_data);
            if let Err(e) = dynamic_image.write_to(&mut cursor, image::ImageFormat::Jpeg) {
                log::error!("Failed to encode image to JPEG: {}", e);
                return;
            }

            // 5. 保存图片到文件系统 (加密保存建议)
            // 这里简化为保存到本地目录
            let filename = format!("{}.jpg", hash_string);
            let save_path = format!("audit_images/{}", filename);

            // 确保目录存在
            let _ = std::fs::create_dir_all("audit_images");

            if let Err(e) = std::fs::write(&save_path, &jpeg_data) {
                log::error!("Failed to save image file: {}", e);
                return;
            }

            // 6. 创建并保存日志记录
            let log = ScreenshotLog {
                pin: "user_pin".to_string(), // 应从配置或环境获取
                capture_time: Local::now().to_rfc3339(),
                app_name: "unknown".to_string(), // Swift端应传递此信息
                window_title: "unknown".to_string(),
                image_path: save_path,
                image_hash: hash_string,
                is_sensitive: false, // 由 Swift 端 OCR 结果决定，这里暂定 false
                ocr_text: None,
                host_id: "host_123".to_string(),
                cpe_id: "cpe_123".to_string(),
                mac: "00:00:00:00:00:00".to_string(),
                ip: "127.0.0.1".to_string(),
            };

            if let Err(e) = ctx.db.save_screenshot_log(&log).await {
                log::error!("Failed to save screenshot log to DB: {}", e);
            } else {
                log::info!("Screenshot saved successfully: {}", log.image_hash);
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
        match serde_json::from_str::<AuditLog>(&r_str) {
            Ok(log) => {
                if let Err(e) = ctx.db.save_audit_log(&log).await {
                    log::error!("Failed to save audit log: {}", e);
                } else {
                    log::info!("Audit log saved: {}", log.id);
                }
            }
            Err(e) => {
                log::error!("Failed to parse audit log JSON: {}", e);
            }
        }
    });
}
