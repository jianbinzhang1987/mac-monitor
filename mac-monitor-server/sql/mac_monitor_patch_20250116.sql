-- Patch: align monitor_log_behavior schema with mapper fields.
ALTER TABLE monitor_log_behavior
  ADD COLUMN host_id varchar(64) DEFAULT '' COMMENT '主机ID',
  ADD COLUMN mac varchar(64) DEFAULT '' COMMENT 'MAC地址',
  ADD COLUMN ip varchar(64) DEFAULT '' COMMENT 'IP地址';
