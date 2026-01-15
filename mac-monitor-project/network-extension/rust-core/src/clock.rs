use std::sync::atomic::{AtomicI64, Ordering};
use chrono::Utc;

static CLOCK_OFFSET: AtomicI64 = AtomicI64::new(0);

pub struct LogicalClock;

impl LogicalClock {
    /// 获取当前逻辑时间（Unix 时间戳，秒）
    pub fn now() -> i64 {
        let system_now = Utc::now().timestamp();
        let offset = CLOCK_OFFSET.load(Ordering::Relaxed);
        system_now + offset
    }

    /// 获取当前逻辑时间（Unix 时间戳，毫秒）
    pub fn now_ms() -> i64 {
        let system_now = Utc::now().timestamp_millis();
        let offset = CLOCK_OFFSET.load(Ordering::Relaxed);
        // 这里为了精确，offset 也可以改为毫秒级，但需求提到“定时请求时间”，通常秒级足够
        system_now + (offset * 1000)
    }

    /// 根据管理端时间校准时钟
    pub fn sync(server_time: i64) {
        let system_now = Utc::now().timestamp();
        let offset = server_time - system_now;
        CLOCK_OFFSET.store(offset, Ordering::Relaxed);
        log::info!("Logical clock synced. Offset: {}s", offset);
    }
}
