CREATE TABLE IF NOT EXISTS audit_logs (
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
);

CREATE TABLE IF NOT EXISTS behavior_logs (
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
);

CREATE TABLE IF NOT EXISTS screenshot_logs (
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
);

CREATE INDEX IF NOT EXISTS idx_audit_uploaded ON audit_logs(is_uploaded);
CREATE INDEX IF NOT EXISTS idx_behavior_uploaded ON behavior_logs(is_uploaded);
CREATE INDEX IF NOT EXISTS idx_screenshot_uploaded ON screenshot_logs(is_uploaded);
CREATE INDEX IF NOT EXISTS idx_screenshot_hash ON screenshot_logs(image_hash);

CREATE TABLE IF NOT EXISTS clipboard_logs (
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
);

CREATE INDEX IF NOT EXISTS idx_clipboard_uploaded ON clipboard_logs(is_uploaded);
