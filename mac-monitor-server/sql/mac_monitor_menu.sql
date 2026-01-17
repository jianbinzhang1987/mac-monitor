-- 菜单 SQL
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('Mac监控', 0, 10, 'mac-monitor', null, 1, 0, 'M', '0', '0', '', 'monitor', 'admin', sysdate(), '', null, 'Mac监控根目录');

set @parentId = @@identity;

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('终端管理', @parentId, 1, 'device', 'monitor/device/index', 1, 0, 'C', '0', '0', 'monitor:device:list', 'server', 'admin', sysdate(), '', null, '终端管理菜单');

set @menuId = @@identity;

-- 按钮 SQL
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('终端查询', @menuId, 1, '#', '', 1, 0, 'F', '0', '0', 'monitor:device:query', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('终端新增', @menuId, 2, '#', '', 1, 0, 'F', '0', '0', 'monitor:device:add', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('终端修改', @menuId, 3, '#', '', 1, 0, 'F', '0', '0', 'monitor:device:edit', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('终端删除', @menuId, 4, '#', '', 1, 0, 'F', '0', '0', 'monitor:device:remove', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('终端导出', @menuId, 5, '#', '', 1, 0, 'F', '0', '0', 'monitor:device:export', '#', 'admin', sysdate(), '', null, '');

-- ----------------------------
-- 剪贴板日志菜单 SQL
-- ----------------------------

-- 获取父菜单ID (审计日志目录)
select @logMenuId := menu_id from sys_menu where menu_name = '审计日志' limit 1;

-- 插入剪贴板日志菜单
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('剪贴板日志', @logMenuId, 4, 'clipboard', 'monitor/log/clipboard/index', 1, 0, 'C', '0', '0', 'monitor:log:clipboard:list', 'clipboard', 'admin', sysdate(), '', null, '剪贴板审计菜单');

set @clipboardMenuId = @@identity;

-- 剪贴板日志按钮权限
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('剪贴板查询', @clipboardMenuId, 1, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:clipboard:query', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('剪贴板删除', @clipboardMenuId, 2, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:clipboard:remove', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('剪贴板导出', @clipboardMenuId, 3, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:clipboard:export', '#', 'admin', sysdate(), '', null, '');
