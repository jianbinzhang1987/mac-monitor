-- 策略管理 菜单 SQL

-- 获取父菜单ID (假设 'Mac监控' 已存在)
select @parentId := menu_id from sys_menu where menu_name = 'Mac监控' limit 1;

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('策略管理', @parentId, 2, 'policy', 'monitor/policy/index', 1, 0, 'C', '0', '0', 'monitor:policy:list', 'system', 'admin', sysdate(), '', null, '策略管理菜单');

set @menuId = @@identity;

-- 按钮 SQL
insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('策略查询', @menuId, 1, '#', '', 1, 0, 'F', '0', '0', 'monitor:policy:query', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('策略新增', @menuId, 2, '#', '', 1, 0, 'F', '0', '0', 'monitor:policy:add', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('策略修改', @menuId, 3, '#', '', 1, 0, 'F', '0', '0', 'monitor:policy:edit', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('策略删除', @menuId, 4, '#', '', 1, 0, 'F', '0', '0', 'monitor:policy:remove', '#', 'admin', sysdate(), '', null, '');

insert into sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark)
values('策略导出', @menuId, 5, '#', '', 1, 0, 'F', '0', '0', 'monitor:policy:export', '#', 'admin', sysdate(), '', null, '');
