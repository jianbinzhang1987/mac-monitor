-- ----------------------------
-- monitor_log_clipboard 剪贴板审计日志表
-- ----------------------------
DROP TABLE IF EXISTS `monitor_log_clipboard`;
CREATE TABLE `monitor_log_clipboard` (
  `log_id`              bigint(20)      NOT NULL AUTO_INCREMENT COMMENT '日志ID',
  `serial_number`       varchar(64)     DEFAULT ''      COMMENT '设备序列号',
  `device_id`           bigint(20)      DEFAULT NULL    COMMENT '设备ID',
  `op_time`             datetime                        COMMENT '操作时间',
  `app_name`            varchar(128)    DEFAULT ''      COMMENT '应用名称',
  `bundle_id`           varchar(128)    DEFAULT ''      COMMENT '应用包名',
  `content`             text                            COMMENT '剪贴板内容',
  `content_type`        varchar(32)     DEFAULT ''      COMMENT '内容类型',
  `risk_level`          int(11)         DEFAULT 0       COMMENT '风险等级(0-2)',
  `host_id`             varchar(64)     DEFAULT ''      COMMENT '主机ID',
  `mac`                 varchar(64)     DEFAULT ''      COMMENT 'MAC地址',
  `ip`                  varchar(64)     DEFAULT ''      COMMENT 'IP地址',
  `create_time`         datetime                        COMMENT '入库时间',
  PRIMARY KEY (`log_id`),
  KEY `idx_serial_number` (`serial_number`),
  KEY `idx_op_time` (`op_time`)
) ENGINE=InnoDB AUTO_INCREMENT=1 COMMENT='剪贴板审计日志表';

-- ----------------------------
-- 插入菜单 (Clipboard Audit)
-- ----------------------------
-- 假设父菜单 '审计日志' ID 为 2000 (需根据实际环境调整，这里仅供参考)
-- INSERT INTO `sys_menu` VALUES (2004, '剪贴板日志', 2000, 4, 'clipboard', 'monitor/log/clipboard/index', '', 1, 0, 'C', '0', '0', 'monitor:log:clipboard:list', 'clipboard', 'admin', sysdate(), '', null, '剪贴板审计菜单');
