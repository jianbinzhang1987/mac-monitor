-- 1. 创建图书表 (支持树形结构)
create table if not exists sys_book (
  book_id           bigint(20)      not null auto_increment    comment '图书ID',
  parent_id         bigint(20)      default 0                  comment '父图书ID',
  ancestors         varchar(50)     default ''                 comment '祖级列表',
  book_name         varchar(100)    not null                   comment '图书名称',
  order_num         int(4)          default 0                  comment '显示顺序',
  author            varchar(64)     default null               comment '作者',
  price             decimal(10,2)   default 0.00               comment '价格',
  status            char(1)         default '0'                comment '状态（0正常 1停用）',
  category          varchar(32)     default ''                 comment '图书分类',
  cover_url         varchar(255)    default ''                 comment '图书封面',
  create_by         varchar(64)     default ''                 comment '创建者',
  create_time 	    datetime                                   comment '创建时间',
  update_by         varchar(64)     default ''                 comment '更新者',
  update_time       datetime                                   comment '更新时间',
  remark            varchar(500)    default null               comment '备注',
  primary key (book_id)
) engine=innodb auto_increment=100 comment = '图书管理表';

-- 2. 创建章节表 (主子表关联)
create table if not exists sys_book_chapter (
  chapter_id        bigint(20)      not null auto_increment    comment '章节ID',
  book_id           bigint(20)      not null                   comment '图书ID',
  chapter_title     varchar(255)    not null                   comment '章节标题',
  order_num         int(4)          default 0                  comment '排序',
  content_summary   text                                       comment '内容摘要',
  primary key (chapter_id)
) engine=innodb comment = '图书章节表';

-- 3. 插入菜单权限 (使用存储过程或简单逻辑防止重复插入)
-- 注意：这里使用 INSERT IGNORE 或根据名称判断，防止每次启动都插入重复菜单
INSERT IGNORE INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, remark)
SELECT 2000, '图书管理', '1', '10', 'book', 'library/book/index', 1, 0, 'C', '0', '0', 'library:book:list', 'documentation', 'admin', sysdate(), '图书管理菜单'
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_name = '图书管理');

-- 4. 插入按钮权限 (基于父菜单 ID)
INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '图书查询', 2000, '1', '#', '', 1, 0, 'F', '0', '0', 'library:book:query', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'library:book:query');

INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '图书新增', 2000, '2', '#', '', 1, 0, 'F', '0', '0', 'library:book:add', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'library:book:add');

INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '图书修改', 2000, '3', '#', '', 1, 0, 'F', '0', '0', 'library:book:edit', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'library:book:edit');

INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '图书删除', 2000, '4', '#', '', 1, 0, 'F', '0', '0', 'library:book:remove', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'library:book:remove');

INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '图书导出', 2000, '5', '#', '', 1, 0, 'F', '0', '0', 'library:book:export', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'library:book:export');

INSERT IGNORE INTO sys_menu (menu_name, parent_id, order_num, path, component, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time)
SELECT '图书导入', 2000, '6', '#', '', 1, 0, 'F', '0', '0', 'library:book:import', '#', 'admin', sysdate()
FROM DUAL WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE perms = 'library:book:import');

-- 5. 初始化图书分类字典
INSERT IGNORE INTO sys_dict_type (dict_name, dict_type, status, create_by, create_time, remark)
VALUES ('图书分类', 'sys_book_category', '0', 'admin', sysdate(), '图书分类列表');

INSERT IGNORE INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_by, create_time, remark)
VALUES (1, '科幻小说', '1', 'sys_book_category', '', 'primary', 'Y', '0', 'admin', sysdate(), '科幻类书籍'),
       (2, '技术手册', '2', 'sys_book_category', '', 'success', 'N', '0', 'admin', sysdate(), '技术类书籍'),
       (3, '文学名著', '3', 'sys_book_category', '', 'info', 'N', '0', 'admin', sysdate(), '文学类书籍'),
       (4, '历史传记', '4', 'sys_book_category', '', 'warning', 'N', '0', 'admin', sysdate(), '历史类书籍');
