-- 1. 创建资产表
create table if not exists biz_asset (
  asset_id          bigint(20)      not null auto_increment    comment '资产ID',
  asset_code        varchar(64)     not null                   comment '资产编号',
  asset_name        varchar(100)    not null                   comment '资产名称',
  asset_type        varchar(2)      default ''                 comment '资产类型',
  status            char(1)         default '0'                comment '状态（0闲置 1在使用 2维修 3报废）',
  purchase_date     date                                       comment '购入日期',
  price             decimal(10,2)   default 0.00               comment '单价',
  location          varchar(100)    default ''                 comment '存放地点',
  manager           varchar(64)     default ''                 comment '管理人',
  del_flag          char(1)         default '0'                comment '删除标志（0代表存在 2代表删除）',
  create_by         varchar(64)     default ''                 comment '创建者',
  create_time 	    datetime                                   comment '创建时间',
  update_by         varchar(64)     default ''                 comment '更新者',
  update_time       datetime                                   comment '更新时间',
  remark            varchar(500)    default null               comment '备注',
  primary key (asset_id)
) engine=innodb auto_increment=100 comment = '资产信息表';

-- 2. 初始化菜单权限
-- 主菜单：资产管理
INSERT IGNORE INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, remark)
SELECT 3000, '资产管理', '0', '10', 'asset', NULL, 1, 0, 'M', '0', '0', '', 'shopping', 'admin', sysdate(), '资产管理目录'
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 3000);

-- 子菜单：资产维护
INSERT IGNORE INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, remark)
SELECT 3001, '资产维护', '3000', '1', 'info', 'asset/info/index', 1, 0, 'C', '0', '0', 'asset:info:list', 'list', 'admin', sysdate(), '资产维护菜单'
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 3001);

-- 按钮权限
INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '资产查询', 3001, '1', '#', '', 1, 0, 'F', '0', '0', 'asset:info:query', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'asset:info:query');

INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '资产新增', 3001, '2', '#', '', 1, 0, 'F', '0', '0', 'asset:info:add', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'asset:info:add');

INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '资产修改', 3001, '3', '#', '', 1, 0, 'F', '0', '0', 'asset:info:edit', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'asset:info:edit');

INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '资产删除', 3001, '4', '#', '', 1, 0, 'F', '0', '0', 'asset:info:remove', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'asset:info:remove');

INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '资产导出', 3001, '5', '#', '', 1, 0, 'F', '0', '0', 'asset:info:export', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'asset:info:export');

-- 3. 初始化字典
-- 资产状态
INSERT IGNORE INTO sys_dict_type (dict_name, dict_type, status, create_by, create_time, remark)
VALUES ('资产状态', 'biz_asset_status', '0', 'admin', sysdate(), '资产状态列表');

INSERT IGNORE INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_by, create_time, remark)
VALUES
(1, '闲置', '0', 'biz_asset_status', '', 'info', 'Y', '0', 'admin', sysdate(), '闲置状态'),
(2, '在使用', '1', 'biz_asset_status', '', 'success', 'N', '0', 'admin', sysdate(), '在使用状态'),
(3, '维修中', '2', 'biz_asset_status', '', 'warning', 'N', '0', 'admin', sysdate(), '维修状态'),
(4, '已报废', '3', 'biz_asset_status', '', 'danger', 'N', '0', 'admin', sysdate(), '报废状态');

-- 资产类型
INSERT IGNORE INTO sys_dict_type (dict_name, dict_type, status, create_by, create_time, remark)
VALUES ('资产类型', 'biz_asset_type', '0', 'admin', sysdate(), '资产类型列表');

INSERT IGNORE INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_by, create_time, remark)
VALUES
(1, '电子设备', '1', 'biz_asset_type', '', 'default', 'Y', '0', 'admin', sysdate(), '电脑、打印机等'),
(2, '办公家具', '2', 'biz_asset_type', '', 'default', 'N', '0', 'admin', sysdate(), '桌椅、柜子等'),
(3, '交通工具', '3', 'biz_asset_type', '', 'default', 'N', '0', 'admin', sysdate(), '车辆等');
