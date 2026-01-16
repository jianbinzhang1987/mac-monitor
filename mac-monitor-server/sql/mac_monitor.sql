-- ----------------------------
-- mac_monitor 客户端管理表
-- ----------------------------
DROP TABLE IF EXISTS `monitor_device`;
CREATE TABLE `monitor_device` (
  `device_id`           bigint(20)      NOT NULL AUTO_INCREMENT COMMENT '设备ID',
  `serial_number`       varchar(64)     DEFAULT ''      COMMENT '序列号',
  `device_name`         varchar(128)    DEFAULT ''      COMMENT '设备名称',
  `os_version`          varchar(32)     DEFAULT ''      COMMENT '系统版本',
  `app_version`         varchar(32)     DEFAULT ''      COMMENT '客户端版本',
  `status`              char(1)         DEFAULT '0'     COMMENT '在线状态(0离线 1在线)',
  `last_heartbeat`      datetime                        COMMENT '最后心跳时间',
  `policy_id`           bigint(20)      DEFAULT NULL    COMMENT '关联策略ID',
  `policy_version`      varchar(32)     DEFAULT ''      COMMENT '策略版本',
  `registered_ip`       varchar(128)    DEFAULT ''      COMMENT '注册IP',
  `create_by`           varchar(64)     DEFAULT ''      COMMENT '创建者',
  `create_time`         datetime                        COMMENT '创建时间',
  `update_by`           varchar(64)     DEFAULT ''      COMMENT '更新者',
  `update_time`         datetime                        COMMENT '更新时间',
  `remark`              varchar(500)    DEFAULT NULL    COMMENT '备注',
  PRIMARY KEY (`device_id`),
  UNIQUE KEY `idx_serial_number` (`serial_number`)
) ENGINE=InnoDB AUTO_INCREMENT=100 COMMENT='监控设备表';

-- ----------------------------
-- monitor_policy 策略表
-- ----------------------------
DROP TABLE IF EXISTS `monitor_policy`;
CREATE TABLE `monitor_policy` (
  `policy_id`           bigint(20)      NOT NULL AUTO_INCREMENT COMMENT '策略ID',
  `policy_name`         varchar(128)    DEFAULT ''      COMMENT '策略名称',
  `traffic_rules`       text                            COMMENT '流量规则(JSON)',
  `screenshot_rules`    text                            COMMENT '截屏规则(JSON)',
  `agent_settings`      text                            COMMENT '客户端设置(JSON)',
  `is_default`          char(1)         DEFAULT '0'     COMMENT '是否默认(1是 0否)',
  `version`             bigint(20)      DEFAULT 1       COMMENT '策略版本(时间戳)',
  `create_by`           varchar(64)     DEFAULT ''      COMMENT '创建者',
  `create_time`         datetime                        COMMENT '创建时间',
  `update_by`           varchar(64)     DEFAULT ''      COMMENT '更新者',
  `update_time`         datetime                        COMMENT '更新时间',
  `remark`              varchar(500)    DEFAULT NULL    COMMENT '备注',
  PRIMARY KEY (`policy_id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 COMMENT='监控策略表';

-- 初始化默认策略
INSERT INTO `monitor_policy` (`policy_name`, `traffic_rules`, `screenshot_rules`, `agent_settings`, `is_default`, `version`, `create_time`)
VALUES ('默认策略', '{"whitelist":["apple.com","google.com"]}', '{"interval":300}', '{"heartbeatInterval":60,"process_blacklist":["clash","v2ray","proxyman","qq"],"app_blacklist":["clash","v2ray","proxyman","qq"]}', '1', 1, sysdate());

-- ----------------------------
-- monitor_log_screenshot 截图日志表
-- ----------------------------
DROP TABLE IF EXISTS `monitor_log_screenshot`;
CREATE TABLE `monitor_log_screenshot` (
  `log_id`              bigint(20)      NOT NULL AUTO_INCREMENT COMMENT '日志ID',
  `device_id`           bigint(20)      DEFAULT NULL    COMMENT '设备ID',
  `serial_number`       varchar(64)     DEFAULT ''      COMMENT '设备序列号',
  `capture_time`        datetime                        COMMENT '截图时间',
  `file_path`           varchar(255)    DEFAULT ''      COMMENT '文件路径',
  `ocr_text`            text                            COMMENT 'OCR识别文本',
  `risk_level`          int(11)         DEFAULT 0       COMMENT '风险等级(0-2)',
  `host_id`             varchar(64)     DEFAULT ''      COMMENT '主机ID',
  `mac`                 varchar(64)     DEFAULT ''      COMMENT 'MAC地址',
  `ip`                  varchar(64)     DEFAULT ''      COMMENT 'IP地址',
  `create_time`         datetime                        COMMENT '入库时间',
  PRIMARY KEY (`log_id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 COMMENT='截图日志表';

-- ----------------------------
-- monitor_log_traffic 流量审计日志表
-- ----------------------------
DROP TABLE IF EXISTS `monitor_log_traffic`;
CREATE TABLE `monitor_log_traffic` (
  `log_id`              bigint(20)      NOT NULL AUTO_INCREMENT COMMENT '日志ID',
  `serial_number`       varchar(64)     DEFAULT ''      COMMENT '设备序列号',
  `audit_time`          datetime                        COMMENT '审计时间',
  `url`                 text                            COMMENT '请求URL',
  `method`              varchar(16)     DEFAULT ''      COMMENT 'HTTP方法',
  `domain`              varchar(255)    DEFAULT ''      COMMENT '域名',
  `process_name`        varchar(128)    DEFAULT ''      COMMENT '进程名',
  `risk_level`          int(11)         DEFAULT 0       COMMENT '风险等级(0-2)',
  `create_time`         datetime                        COMMENT '入库时间',
  PRIMARY KEY (`log_id`),
  KEY `idx_serial_number` (`serial_number`)
) ENGINE=InnoDB AUTO_INCREMENT=1 COMMENT='流量审计日志表';

-- ----------------------------
-- monitor_log_behavior 行为审计日志表
-- ----------------------------
DROP TABLE IF EXISTS `monitor_log_behavior`;
CREATE TABLE `monitor_log_behavior` (
  `log_id`              bigint(20)      NOT NULL AUTO_INCREMENT COMMENT '日志ID',
  `serial_number`       varchar(64)     DEFAULT ''      COMMENT '设备序列号',
  `event_time`          datetime                        COMMENT '事件时间',
  `event_type`          varchar(32)     DEFAULT ''      COMMENT '事件类型(PROCESS_LAUNCH, USB_INSERT, FILE_CHANGE)',
  `process_name`        varchar(128)    DEFAULT ''      COMMENT '相关进程',
  `detail`              text                            COMMENT '详细描述(JSON)',
  `risk_level`          int(11)         DEFAULT 0       COMMENT '风险等级(0-2)',
  `host_id`             varchar(64)     DEFAULT ''      COMMENT '主机ID',
  `mac`                 varchar(64)     DEFAULT ''      COMMENT 'MAC地址',
  `ip`                  varchar(64)     DEFAULT ''      COMMENT 'IP地址',
  `create_time`         datetime                        COMMENT '入库时间',
  PRIMARY KEY (`log_id`),
  KEY `idx_serial_number` (`serial_number`),
  KEY `idx_event_type` (`event_type`)
) ENGINE=InnoDB AUTO_INCREMENT=1 COMMENT='行为审计日志表';
