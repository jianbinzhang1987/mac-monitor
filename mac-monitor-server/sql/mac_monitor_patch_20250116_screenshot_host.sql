-- Patch: add host_id/mac/ip to monitor_log_screenshot.
ALTER TABLE monitor_log_screenshot
  ADD COLUMN host_id varchar(64) DEFAULT '' COMMENT '主机ID',
  ADD COLUMN mac varchar(64) DEFAULT '' COMMENT 'MAC地址',
  ADD COLUMN ip varchar(64) DEFAULT '' COMMENT 'IP地址';
