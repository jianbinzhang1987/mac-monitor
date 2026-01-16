-- Patch: add app_name and image_hash to monitor_log_screenshot
ALTER TABLE monitor_log_screenshot
  ADD COLUMN app_name varchar(128) DEFAULT '' COMMENT '应用名称',
  ADD COLUMN image_hash varchar(64) DEFAULT '' COMMENT '图片哈希';
