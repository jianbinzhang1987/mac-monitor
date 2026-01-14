use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::str::FromStr;
use crate::models::{AuditLog, BehaviorLog, ScreenshotLog};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::from_str(db_path)?
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        let db = Self { pool };
        db.init().await?;

        Ok(db)
    }

    async fn init(&self) -> Result<(), sqlx::Error> {
        // 创建审计日志表
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS audit_logs (
                id TEXT PRIMARY KEY,
                pin_number TEXT,
                url TEXT,
                req_time TEXT,
                resp_time TEXT,
                method_type TEXT,
                ip TEXT,
                mac TEXT,
                cpe_id TEXT,
                host_id TEXT,
                status_code TEXT,
                request_body TEXT,
                response_body TEXT,
                title TEXT,
                is_uploaded INTEGER DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&self.pool).await?;

        // 创建行为日志表
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS behavior_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                proc TEXT,
                op_time TEXT,
                pin TEXT,
                op_file TEXT,
                op_type TEXT,
                op_ret TEXT,
                op_reason TEXT,
                host_id TEXT,
                cpe_id TEXT,
                mac TEXT,
                ip TEXT,
                is_uploaded INTEGER DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&self.pool).await?;

        // 创建截图日志表
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS screenshot_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                pin TEXT,
                capture_time TEXT,
                app_name TEXT,
                window_title TEXT,
                image_path TEXT,
                image_hash TEXT,
                is_sensitive INTEGER,
                ocr_text TEXT,
                host_id TEXT,
                cpe_id TEXT,
                mac TEXT,
                ip TEXT,
                is_uploaded INTEGER DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&self.pool).await?;

        // 创建索引以提升查询性能
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_uploaded ON audit_logs(is_uploaded)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_behavior_uploaded ON behavior_logs(is_uploaded)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_screenshot_uploaded ON screenshot_logs(is_uploaded)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_screenshot_hash ON screenshot_logs(image_hash)")
            .execute(&self.pool).await?;

        Ok(())
    }

    pub async fn save_audit_log(&self, log: &AuditLog) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO audit_logs (id, pin_number, url, req_time, resp_time, method_type, ip, mac, cpe_id, host_id, status_code, request_body, response_body, title)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&log.id)
        .bind(&log.pin_number)
        .bind(&log.url)
        .bind(&log.req_time)
        .bind(&log.resp_time)
        .bind(&log.method_type)
        .bind(&log.ip)
        .bind(&log.mac)
        .bind(&log.cpe_id)
        .bind(&log.host_id)
        .bind(&log.status_code)
        .bind(&log.request_body)
        .bind(&log.response_body)
        .bind(&log.title)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_behavior_log(&self, log: &BehaviorLog) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO behavior_logs (proc, op_time, pin, op_file, op_type, op_ret, op_reason, host_id, cpe_id, mac, ip)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&log.proc)
        .bind(&log.op_time)
        .bind(&log.pin)
        .bind(&log.op_file)
        .bind(&log.op_type)
        .bind(&log.op_ret)
        .bind(&log.op_reason)
        .bind(&log.host_id)
        .bind(&log.cpe_id)
        .bind(&log.mac)
        .bind(&log.ip)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_screenshot_log(&self, log: &ScreenshotLog) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO screenshot_logs (pin, capture_time, app_name, window_title, image_path, image_hash, is_sensitive, ocr_text, host_id, cpe_id, mac, ip)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&log.pin)
        .bind(&log.capture_time)
        .bind(&log.app_name)
        .bind(&log.window_title)
        .bind(&log.image_path)
        .bind(&log.image_hash)
        .bind(log.is_sensitive as i32)
        .bind(&log.ocr_text)
        .bind(&log.host_id)
        .bind(&log.cpe_id)
        .bind(&log.mac)
        .bind(&log.ip)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_unsent_audit_logs(&self) -> Result<Vec<AuditLog>, sqlx::Error> {
        sqlx::query_as!(
            AuditLog,
            "SELECT pin_number, id, url, req_time, resp_time, method_type, ip, mac, cpe_id, host_id, status_code, request_body, response_body, title FROM audit_logs WHERE is_uploaded = 0 LIMIT 10"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn mark_audit_log_sent(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE audit_logs SET is_uploaded = 1 WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_unsent_behavior_logs(&self) -> Result<Vec<BehaviorLog>, sqlx::Error> {
        sqlx::query_as!(
            BehaviorLog,
            "SELECT proc, op_time, pin, op_file, op_type, op_ret, op_reason, host_id, cpe_id, mac, ip FROM behavior_logs WHERE is_uploaded = 0 LIMIT 10"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn mark_behavior_log_sent(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE behavior_logs SET is_uploaded = 1 WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_unsent_screenshot_logs(&self) -> Result<Vec<ScreenshotLog>, sqlx::Error> {
        sqlx::query_as!(
            ScreenshotLog,
            "SELECT pin, capture_time, app_name, window_title, image_path, image_hash, is_sensitive, ocr_text, host_id, cpe_id, mac, ip FROM screenshot_logs WHERE is_uploaded = 0 LIMIT 5"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn mark_screenshot_log_sent(&self, hash: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE screenshot_logs SET is_uploaded = 1 WHERE image_hash = ?", hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn check_screenshot_exists(&self, hash: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM screenshot_logs WHERE image_hash = ?", hash)
            .fetch_one(&self.pool)
            .await?;
        Ok(result.count > 0)
    }
}
