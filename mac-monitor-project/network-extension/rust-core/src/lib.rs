mod device;
mod stack;
mod mitm;

pub use device::TunDevice;
pub use stack::{NetworkStack, StackHandle};
pub use mitm::MitmProxy;

use std::os::raw::{c_char, c_void};
use std::sync::{Arc, Mutex};
use smoltcp::wire::{IpProtocol, Ipv4Packet, TcpPacket, UdpPacket};

lazy_static::lazy_static! {
    static ref GLOBAL_STACK: Arc<Mutex<Option<StackHandle>>> = Arc::new(Mutex::new(None));
}

#[no_mangle]
pub extern "C" fn init_stack() -> i32 {
    log::info!("Initializing user-space TCP/IP stack with smoltcp...");

    match NetworkStack::new() {
        Ok(stack_handle) => {
            let mut global = GLOBAL_STACK.lock().unwrap();
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

    let global = GLOBAL_STACK.lock().unwrap();
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

    let global = GLOBAL_STACK.lock().unwrap();
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
    let global = GLOBAL_STACK.lock().unwrap();
    if let Some(ref stack) = *global {
        stack.poll();
    }
}

#[no_mangle]
pub extern "C" fn shutdown_stack() {
    let mut global = GLOBAL_STACK.lock().unwrap();
    *global = None;
    log::info!("Network stack shutdown");
}
