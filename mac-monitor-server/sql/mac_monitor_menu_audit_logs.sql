-- 流量审计与行为审计 菜单 SQL

-- 获取父菜单ID (审计日志目录)
select @logMenuId := menu_id from sys_menu where menu_name = '审计日志' limit 1;

-- 1. 流量审计菜单
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('流量审计', @logMenuId, 2, 'traffic', 'monitor/log/traffic/index', 1, 0, 'C', '0', '0', 'monitor:log:traffic:list', 'guide', 'admin', sysdate(), '', null, '流量审计菜单');

set @trafficMenuId = @@identity;

-- 流量审计按钮权限
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('流量查询', @trafficMenuId, 1, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:traffic:query', '#', 'admin', sysdate(), '', null, '');
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('流量删除', @trafficMenuId, 2, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:traffic:remove', '#', 'admin', sysdate(), '', null, '');
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('流量导出', @trafficMenuId, 3, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:traffic:export', '#', 'admin', sysdate(), '', null, '');


-- 2. 行为审计菜单
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('行为审计', @logMenuId, 3, 'behavior', 'monitor/log/behavior/index', 1, 0, 'C', '0', '0', 'monitor:log:behavior:list', 'documentation', 'admin', sysdate(), '', null, '行为审计菜单');

set @behaviorMenuId = @@identity;

-- 行为审计按钮权限
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('行为查询', @behaviorMenuId, 1, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:behavior:query', '#', 'admin', sysdate(), '', null, '');
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('行为删除', @behaviorMenuId, 2, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:behavior:remove', '#', 'admin', sysdate(), '', null, '');
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('行为导出', @behaviorMenuId, 3, '#', '', 1, 0, 'F', '0', '0', 'monitor:log:behavior:export', '#', 'admin', sysdate(), '', null, '');
