use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LogicalClock {
    // Offset in milliseconds between local system time and server time
    offset_ms: AtomicI64,
}

impl LogicalClock {
    pub fn new() -> Self {
        Self {
            offset_ms: AtomicI64::new(0),
        }
    }

    /// Update the offset based on server time
    /// server_time_str: "yyyy-MM-dd HH:mm:ss" as per spec
    pub fn update_offset(&self, server_time_ms: i64) {
        let local_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let new_offset = server_time_ms - local_now;
        self.offset_ms.store(new_offset, Ordering::SeqCst);
        println!("Logical clock offset updated: {}ms", new_offset);
    }

    /// Get current logical time in milliseconds
    pub fn now_ms(&self) -> i64 {
        let local_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        local_now + self.offset_ms.load(Ordering::SeqCst)
    }

    /// Format logical time as string for logging
    pub fn now_str(&self) -> String {
        let now_ms = self.now_ms();
        let seconds = (now_ms / 1000) as i64;
        let nanos = ((now_ms % 1000) * 1_000_000) as u32;

        // Simple formatting or use chrono if added to dependencies
        format!("Timestamp: {}", now_ms)
    }
}
