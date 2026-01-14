-- 1. 更新主菜单组件路径、图标和权限标识
UPDATE sys_menu
SET component = 'library/book/index',
    icon = 'documentation',
    perms = 'library:book:list'
WHERE menu_name = '图书管理';

-- 2. 更新按钮权限标识
UPDATE sys_menu SET perms = 'library:book:query' WHERE perms = 'system:book:query';
UPDATE sys_menu SET perms = 'library:book:add' WHERE perms = 'system:book:add';
UPDATE sys_menu SET perms = 'library:book:edit' WHERE perms = 'system:book:edit';
UPDATE sys_menu SET perms = 'library:book:remove' WHERE perms = 'system:book:remove';
UPDATE sys_menu SET perms = 'library:book:export' WHERE perms = 'system:book:export';
UPDATE sys_menu SET perms = 'library:book:import' WHERE perms = 'system:book:import';

-- 3. 提交更改 (如果是在命令行手动执行)
-- COMMIT;
