use sqlx::{sqlite::SqliteConnectOptions, Row, SqlitePool};
use std::str::FromStr;
use crate::models::{AuditLog, BehaviorLog, ClipboardLog, ScreenshotLog};

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
        // 创建流量审计表 (与服务端对齐)
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS monitor_log_traffic (
                id TEXT PRIMARY KEY,
                cpe_id TEXT,
                url TEXT,
                req_time TEXT,
                method_type TEXT,
                domain TEXT,
                process_name TEXT,
                risk_level INTEGER,
                ip TEXT,
                mac TEXT,
                host_id TEXT,
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
                cpe_id TEXT,
                op_type TEXT,
                detail TEXT,
                risk_level INTEGER,
                host_id TEXT,
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
                capture_time TEXT,
                cpe_id TEXT,
                image_path TEXT,
                ocr_text TEXT,
                risk_level INTEGER,
                app_name TEXT,
                image_hash TEXT,
                host_id TEXT,
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

        // 创建剪贴板日志表
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS clipboard_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                app_name TEXT,
                bundle_id TEXT,
                op_time TEXT,
                content TEXT,
                content_type TEXT,
                risk_level INTEGER,
                host_id TEXT,
                cpe_id TEXT,
                mac TEXT,
                ip TEXT,
                is_uploaded INTEGER DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&self.pool).await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_clipboard_uploaded ON clipboard_logs(is_uploaded)")
            .execute(&self.pool).await?;

        // 数据库迁移：尝试添加新字段 (忽略已存在的错误)
        // Behavior Logs
        let _ = sqlx::query("ALTER TABLE behavior_logs ADD COLUMN host_id TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE behavior_logs ADD COLUMN mac TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE behavior_logs ADD COLUMN ip TEXT").execute(&self.pool).await;

        // Audit Logs
        let _ = sqlx::query("ALTER TABLE audit_logs ADD COLUMN host_id TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE audit_logs ADD COLUMN mac TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE audit_logs ADD COLUMN ip TEXT").execute(&self.pool).await;

        // Screenshot Logs
        let _ = sqlx::query("ALTER TABLE screenshot_logs ADD COLUMN host_id TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE screenshot_logs ADD COLUMN mac TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE screenshot_logs ADD COLUMN ip TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE screenshot_logs ADD COLUMN redaction_labels TEXT").execute(&self.pool).await;

        Ok(())
    }

    pub async fn save_audit_log(&self, log: &AuditLog) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO monitor_log_traffic (id, cpe_id, url, req_time, method_type, domain, process_name, risk_level, ip, mac, host_id)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&log.id)
        .bind(&log.cpe_id)
        .bind(&log.url)
        .bind(&log.req_time)
        .bind(&log.method_type)
        .bind(&log.domain)
        .bind(&log.process_name)
        .bind(log.risk_level)
        .bind(&log.ip)
        .bind(&log.mac)
        .bind(&log.host_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_behavior_log(&self, log: &BehaviorLog) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO behavior_logs (proc, op_time, cpe_id, op_type, detail, risk_level, host_id, mac, ip)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&log.proc)
        .bind(&log.op_time)
        .bind(&log.cpe_id)
        .bind(&log.op_type)
        .bind(&log.detail)
        .bind(log.risk_level)
        .bind(&log.host_id)
        .bind(&log.mac)
        .bind(&log.ip)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_screenshot_log(&self, log: &ScreenshotLog) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO screenshot_logs (capture_time, cpe_id, image_path, ocr_text, risk_level, app_name, image_hash, host_id, mac, ip, redaction_labels)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&log.capture_time)
        .bind(&log.cpe_id)
        .bind(&log.image_path)
        .bind(&log.ocr_text)
        .bind(log.risk_level)
        .bind(&log.app_name)
        .bind(&log.image_hash)
        .bind(&log.host_id)
        .bind(&log.mac)
        .bind(&log.ip)
        .bind(&log.redaction_labels)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_clipboard_log(&self, log: &ClipboardLog) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO clipboard_logs (app_name, bundle_id, op_time, content, content_type, risk_level, host_id, cpe_id, mac, ip)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&log.app_name)
        .bind(&log.bundle_id)
        .bind(&log.op_time)
        .bind(&log.content)
        .bind(&log.content_type)
        .bind(log.risk_level)
        .bind(&log.host_id)
        .bind(&log.cpe_id)
        .bind(&log.mac)
        .bind(&log.ip)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_unsent_audit_logs(&self) -> Result<Vec<AuditLog>, sqlx::Error> {
        let rows = sqlx::query(
            r#"SELECT
                cpe_id,
                id,
                url,
                req_time,
                method_type,
                domain,
                process_name,
                risk_level,
                ip,
                mac,
                host_id
            FROM monitor_log_traffic WHERE is_uploaded = 0 LIMIT 10"#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::with_capacity(rows.len());
        for row in rows {
            logs.push(AuditLog {
                cpe_id: row.try_get("cpe_id")?,
                id: row.try_get("id")?,
                url: row.try_get("url")?,
                req_time: row.try_get("req_time")?,
                method_type: row.try_get("method_type")?,
                domain: row.try_get("domain")?,
                process_name: row.try_get("process_name")?,
                risk_level: row.try_get("risk_level")?,
                ip: row.try_get("ip")?,
                mac: row.try_get("mac")?,
                host_id: row.try_get("host_id")?,
            });
        }
        Ok(logs)
    }

    pub async fn mark_audit_log_sent(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE monitor_log_traffic SET is_uploaded = 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_unsent_behavior_logs(&self) -> Result<Vec<BehaviorLog>, sqlx::Error> {
        let rows = sqlx::query(
            r#"SELECT
                id,
                proc,
                op_time,
                cpe_id,
                op_type,
                detail,
                risk_level,
                host_id,
                mac,
                ip
            FROM behavior_logs WHERE is_uploaded = 0 LIMIT 10"#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::with_capacity(rows.len());
        for row in rows {
            logs.push(BehaviorLog {
                id: Some(row.try_get::<i64, _>("id")?),
                proc: row.try_get("proc")?,
                op_time: row.try_get("op_time")?,
                cpe_id: row.try_get("cpe_id")?,
                op_type: row.try_get("op_type")?,
                detail: row.try_get("detail")?,
                risk_level: row.try_get("risk_level")?,
                host_id: row.try_get("host_id")?,
                mac: row.try_get("mac")?,
                ip: row.try_get("ip")?,
            });
        }
        Ok(logs)
    }

    pub async fn mark_behavior_log_sent(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE behavior_logs SET is_uploaded = 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_unsent_screenshot_logs(&self) -> Result<Vec<ScreenshotLog>, sqlx::Error> {
        let rows = sqlx::query(
            r#"SELECT
                id,
                capture_time,
                cpe_id,
                image_path,
                ocr_text,
                risk_level,
                app_name,
                image_hash,
                host_id,
                mac,
                ip,
                redaction_labels
            FROM screenshot_logs WHERE is_uploaded = 0 LIMIT 5"#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::with_capacity(rows.len());
        for row in rows {
            logs.push(ScreenshotLog {
                id: Some(row.try_get::<i64, _>("id")?),
                capture_time: row.try_get("capture_time")?,
                cpe_id: row.try_get("cpe_id")?,
                image_path: row.try_get("image_path")?,
                ocr_text: row.try_get::<Option<String>, _>("ocr_text")?,
                risk_level: row.try_get("risk_level")?,
                app_name: row.try_get("app_name")?,
                image_hash: row.try_get("image_hash")?,
                host_id: row.try_get("host_id")?,
                mac: row.try_get("mac")?,
                ip: row.try_get("ip")?,
                redaction_labels: row.try_get::<Option<String>, _>("redaction_labels")?,
            });
        }
        Ok(logs)
    }

    pub async fn mark_screenshot_log_sent(&self, hash: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE screenshot_logs SET is_uploaded = 1 WHERE image_hash = ?")
            .bind(hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn check_screenshot_exists(&self, hash: &str) -> Result<bool, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM screenshot_logs WHERE image_hash = ?")
            .bind(hash)
            .fetch_one(&self.pool)
            .await?;
        let count: i64 = row.try_get("count")?;
        Ok(count > 0)
    }

    pub async fn get_unsent_clipboard_logs(&self) -> Result<Vec<ClipboardLog>, sqlx::Error> {
        let rows = sqlx::query(
            r#"SELECT
                id,
                app_name,
                bundle_id,
                op_time,
                content,
                content_type,
                risk_level,
                cpe_id,
                host_id,
                mac,
                ip
            FROM clipboard_logs WHERE is_uploaded = 0 LIMIT 20"#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::with_capacity(rows.len());
        for row in rows {
            logs.push(ClipboardLog {
                id: Some(row.try_get::<i64, _>("id")?),
                app_name: row.try_get("app_name")?,
                bundle_id: row.try_get("bundle_id")?,
                op_time: row.try_get("op_time")?,
                content: row.try_get("content")?,
                content_type: row.try_get("content_type")?,
                risk_level: row.try_get("risk_level")?,
                cpe_id: row.try_get("cpe_id")?,
                host_id: row.try_get("host_id")?,
                mac: row.try_get("mac")?,
                ip: row.try_get("ip")?,
            });
        }
        Ok(logs)
    }

    pub async fn mark_clipboard_log_sent(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE clipboard_logs SET is_uploaded = 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_all_clipboard_logs(&self) -> Result<Vec<ClipboardLog>, sqlx::Error> {
        let rows = sqlx::query(
            r#"SELECT
                id,
                app_name,
                bundle_id,
                op_time,
                content,
                content_type,
                risk_level,
                cpe_id,
                host_id,
                mac,
                ip
            FROM clipboard_logs ORDER BY op_time DESC LIMIT 100"#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::with_capacity(rows.len());
        for row in rows {
            logs.push(ClipboardLog {
                id: Some(row.try_get::<i64, _>("id")?),
                app_name: row.try_get("app_name")?,
                bundle_id: row.try_get("bundle_id")?,
                op_time: row.try_get("op_time")?,
                content: row.try_get("content")?,
                content_type: row.try_get("content_type")?,
                risk_level: row.try_get("risk_level")?,
                cpe_id: row.try_get("cpe_id")?,
                host_id: row.try_get("host_id")?,
                mac: row.try_get("mac")?,
                ip: row.try_get("ip")?,
            });
        }
        Ok(logs)
    }
}
