mod device;
mod stack;
mod mitm;
mod clock;
pub mod dns;

pub use device::TunDevice;
pub use stack::{NetworkStack, StackHandle};
pub use mitm::MitmProxy;
pub use mitm::DeviceInfo;
pub use clock::LogicalClock;

use std::ffi::CStr;
use std::io::Write;
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref GLOBAL_STACK: Arc<Mutex<Option<StackHandle>>> = Arc::new(Mutex::new(None));
}

#[no_mangle]
pub extern "C" fn init_stack() -> i32 {
    log::info!("Initializing user-space TCP/IP stack with smoltcp...");

    match NetworkStack::new() {
        Ok(stack_handle) => {
            let mut global = (*GLOBAL_STACK).lock().unwrap();
            *global = Some(stack_handle);
            log::info!("Network stack initialized successfully");
            0
        }
        Err(e) => {
            log::error!("Failed to initialize network stack: {:?}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn process_packet(data: *const u8, len: usize) -> i32 {
    if data.is_null() || len == 0 {
        return -1;
    }

    let packet_data = unsafe { std::slice::from_raw_parts(data, len) };

    let global = (*GLOBAL_STACK).lock().unwrap();
    if let Some(ref stack) = *global {
        match stack.process_inbound_packet(packet_data) {
            Ok(_) => 0,
            Err(e) => {
                log::error!("Failed to process packet: {:?}", e);
                -1
            }
        }
    } else {
        log::error!("Network stack not initialized");
        -1
    }
}

#[no_mangle]
pub extern "C" fn get_outbound_packet(buffer: *mut u8, buffer_len: usize, written_len: *mut usize) -> i32 {
    if buffer.is_null() || written_len.is_null() {
        return -1;
    }

    let global = (*GLOBAL_STACK).lock().unwrap();
    if let Some(ref stack) = *global {
        let out_buffer = unsafe { std::slice::from_raw_parts_mut(buffer, buffer_len) };
        match stack.get_outbound_packet(out_buffer) {
            Ok(len) => {
                unsafe { *written_len = len; }
                0
            }
            Err(_) => -1
        }
    } else {
        -1
    }
}

#[no_mangle]
pub extern "C" fn poll_stack() {
    let global = (*GLOBAL_STACK).lock().unwrap();
    if let Some(ref stack) = *global {
        stack.poll();
    }
}

#[no_mangle]
pub extern "C" fn shutdown_stack() {
    let mut global = (*GLOBAL_STACK).lock().unwrap();
    *global = None;
    log::info!("Network stack shutdown");
}

#[no_mangle]
pub extern "C" fn sync_logic_clock(server_time: i64) {
    LogicalClock::sync(server_time);
}

#[no_mangle]
pub extern "C" fn set_device_info(
    pin_number: *const c_char,
    ip: *const c_char,
    mac: *const c_char,
    cpe_id: *const c_char,
    host_id: *const c_char,
) {
    let info = DeviceInfo {
        pin_number: cstr_to_string(pin_number),
        ip: cstr_to_string(ip),
        mac: cstr_to_string(mac),
        cpe_id: cstr_to_string(cpe_id),
        host_id: cstr_to_string(host_id),
    };

    mitm::set_device_info(info);
}

#[no_mangle]
pub extern "C" fn set_audit_policy(policy_json: *const c_char) -> i32 {
    let json = cstr_to_string(policy_json);
    if json.is_empty() {
        return -1;
    }
    match mitm::set_audit_policy_json(&json) {
        Ok(_) => 0,
        Err(e) => {
            log::error!("Failed to set audit policy: {}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn export_root_ca_pem(buffer: *mut u8, buffer_len: usize) -> i32 {
    if buffer.is_null() {
        return -1;
    }
    
    // 我们在这里临时创建一个 MitmProxy 来生成相同的 CA (实际上应该从全局单例获取)
    let proxy = MitmProxy::new();
    let pem = proxy.get_root_ca_pem();
    let pem_bytes = pem.as_bytes();
    
    if pem_bytes.len() > buffer_len {
        return pem_bytes.len() as i32; // 返回所需长度
    }
    
    unsafe {
        std::ptr::copy_nonoverlapping(pem_bytes.as_ptr(), buffer, pem_bytes.len());
        if pem_bytes.len() < buffer_len {
            *buffer.add(pem_bytes.len()) = 0; // Null terminator
        }
    }
    
    pem_bytes.len() as i32
}

#[no_mangle]
pub extern "C" fn log_audit_event(event_json: *const c_char) {
    let json = cstr_to_string(event_json);
    if json.is_empty() {
        return;
    }
    log_audit_event_internal(&json);
}

#[no_mangle]
pub extern "C" fn analyze_enhanced_image(
    ptr: *const u8,
    len: usize,
    width: u32,
    height: u32,
    app_name: *const c_char,
    is_sensitive: bool,
    ocr_text: *const c_char,
) {
    let app_name = cstr_to_string(app_name);
    let ocr_text = cstr_to_string(ocr_text);
    
    // 构造审计日志
    // 实际生产中应配合 image 库保存 ptr 指向的像素数据为 JPG
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let filename = format!("{}_{}.jpg", app_name, timestamp.replace(" ", "_"));
    let path = format!("/Library/Application Support/MacMonitor/screenshots/{}", filename);

    let log_json = serde_json::json!({
        "type": "screenshot",
        "proc": app_name,
        "op_time": timestamp,
        "op_type": "2",
        "op_reason": ocr_text,
        "op_ret": if is_sensitive { "sensitive_detected" } else { "normal" },
        "file_path": path
    });

    if let Ok(data) = serde_json::to_string(&log_json) {
        log_audit_event_internal(&data);
    }
}

fn log_audit_event_internal(json: &str) {
    let socket_path = "/tmp/mac_monitor_audit.sock";
    use interprocess::local_socket::LocalSocketStream;
    if let Ok(mut stream) = LocalSocketStream::connect(socket_path) {
        let command = serde_json::json!({
            "command": "log_event",
            "payload": json
        });
        if let Ok(data) = serde_json::to_string(&command) {
            let _ = stream.write_all(data.as_bytes());
        }
    }
}

fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}
