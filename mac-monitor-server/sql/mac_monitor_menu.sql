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
