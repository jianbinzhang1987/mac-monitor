-- 审计日志 菜单 SQL

-- 获取父菜单ID (假设 'Mac监控' 已存在)
select @parentId := menu_id from sys_menu where menu_name = 'Mac监控' limit 1;

-- 插入 审计日志 目录
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('审计日志', @parentId, 3, 'log', null, 1, 0, 'M', '0', '0', '', 'log', 'admin', sysdate(), '', null, '审计日志目录');

set @logMenuId = @@identity;

-- 插入 截图日志 菜单
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('截图日志', @logMenuId, 1, 'screenshot', 'monitor/log/screenshot/index', 1, 0, 'C', '0', '0', 'monitor:log:screenshot:list', 'photo', 'admin', sysdate(), '', null, '截图日志菜单');

set @screenshotMenuId = @@identity;

-- 按钮
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('日志查询', @screenshotMenuId, 1, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:screenshot:query', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('日志删除', @screenshotMenuId, 2, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:screenshot:remove', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('日志导出', @screenshotMenuId, 3, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:screenshot:export', '#', 'admin', sysdate(), '', null, '');
