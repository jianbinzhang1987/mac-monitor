# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

RuoYi-Vue 是一个基于 Spring Boot + Vue 的前后端分离快速开发框架（版本 3.9.1）。

**技术栈:**
- 后端: Spring Boot 2.5.15, Spring Security, MyBatis, Redis, JWT
- 前端: Vue 2.6.12, Element UI 2.15.14, Vuex, Vue Router
- 数据库: MySQL (使用 Druid 连接池)
- 代码生成: Velocity 模板引擎
- 定时任务: Quartz

## 常用命令

### 后端开发

```bash
# 编译整个项目（在项目根目录执行）
mvn clean install

# 只编译不运行测试
mvn clean install -DskipTests

# 打包后端应用
cd ruoyi-admin
mvn clean package

# 运行后端服务（开发环境）
# 方式1: 使用 Maven
cd ruoyi-admin
mvn spring-boot:run

# 方式2: 运行打包后的 jar
java -jar ruoyi-admin/target/ruoyi-admin.jar

# 方式3: 使用提供的脚本（生产环境）
./ry.sh start    # 启动
./ry.sh stop     # 停止
./ry.sh restart  # 重启
./ry.sh status   # 查看状态
```

**后端服务默认端口:** 8080
**配置文件位置:** [ruoyi-admin/src/main/resources/application.yml](ruoyi-admin/src/main/resources/application.yml)

### 前端开发

```bash
cd ruoyi-ui

# 安装依赖
npm install
# 或使用国内镜像
npm install --registry=https://registry.npmmirror.com

# 启动开发服务器
npm run dev

# 构建生产环境
npm run build:prod

# 构建测试环境
npm run build:stage
```

**前端开发服务器默认端口:** 80
**开发环境 API 代理:** `/dev-api` 代理到 `http://localhost:8080`

### 数据库初始化

```bash
# 导入主要的数据库脚本
mysql -u root -p your_database < sql/ry_20250522.sql

# 如果使用定时任务功能，还需导入
mysql -u root -p your_database < sql/quartz.sql
```

## 项目架构

### 后端模块结构

项目采用 Maven 多模块架构，父 POM 位于根目录 [pom.xml](pom.xml)：

- **ruoyi-admin**: Web 服务入口模块，包含启动类 [RuoYiApplication.java](ruoyi-admin/src/main/java/com/ruoyi/RuoYiApplication.java)，Controller 层代码
- **ruoyi-framework**: 框架核心模块，包含：
  - Spring Security 安全配置
  - Redis 缓存配置
  - MyBatis 配置
  - 拦截器、切面等基础设施
  - 位置: `ruoyi-framework/src/main/java/com/ruoyi/framework/`
- **ruoyi-system**: 系统业务模块，包含用户、角色、菜单、部门等核心业务的 domain/mapper/service
- **ruoyi-common**: 通用工具模块，包含：
  - 公共实体类（如 AjaxResult, BaseController）
  - 工具类（utils 包）
  - 注解、枚举、异常定义
  - Redis 缓存工具类
- **ruoyi-quartz**: 定时任务模块，基于 Quartz 实现
- **ruoyi-generator**: 代码生成器模块，使用 Velocity 模板生成前后端代码

### 前端目录结构

```
ruoyi-ui/
├── src/
│   ├── api/          # API 请求封装（按业务模块组织）
│   ├── assets/       # 静态资源（图片、样式等）
│   ├── components/   # 全局公共组件
│   ├── directive/    # 自定义指令
│   ├── layout/       # 布局组件
│   ├── router/       # 路由配置
│   ├── store/        # Vuex 状态管理
│   ├── utils/        # 工具函数
│   ├── views/        # 页面组件（按业务模块组织）
│   ├── main.js       # 入口文件
│   └── permission.js # 路由权限控制
├── vue.config.js     # Vue CLI 配置
└── package.json
```

### 核心配置说明

**后端配置:**
- 主配置: [application.yml](ruoyi-admin/src/main/resources/application.yml)
- 数据源配置: [application-druid.yml](ruoyi-admin/src/main/resources/application-druid.yml)
- MyBatis 配置: `ruoyi-admin/src/main/resources/mybatis/mybatis-config.xml`
- Mapper XML 路径: `各模块/src/main/resources/mapper/**/*Mapper.xml`

**前端配置:**
- 开发环境: [.env.development](ruoyi-ui/.env.development) - API 路径为 `/dev-api`
- 生产环境: `.env.production` - API 路径为 `/prod-api`
- Vue CLI 配置: [vue.config.js](ruoyi-ui/vue.config.js) - 包含代理配置和打包优化

## 关键架构概念

### 认证授权流程

- 使用 JWT (JSON Web Token) 进行无状态认证
- Token 配置在 [application.yml](ruoyi-admin/src/main/resources/application.yml) 中（header: Authorization, 默认过期时间 30 分钟）
- Spring Security 配置位于 `ruoyi-framework/src/main/java/com/ruoyi/framework/config/SecurityConfig.java`
- 登录信息缓存在 Redis 中

### 数据权限控制

- 基于角色的访问控制（RBAC）
- 支持按部门/机构的数据范围权限划分
- 权限注解使用 Spring Security 的 `@PreAuthorize`

### 代码生成器

- 访问路径: 系统工具 -> 代码生成
- 可以根据数据库表一键生成 CRUD 的前后端代码（Java domain/mapper/service/controller + Vue 页面）
- 模板位置: `ruoyi-generator/src/main/resources/vm/`
- 使用 Velocity 模板引擎

### 动态多数据源

- 支持多数据源动态切换
- 配置类: `ruoyi-framework/src/main/java/com/ruoyi/framework/datasource/DynamicDataSource.java`
- 通过注解 `@DataSource` 切换数据源

### 前后端交互

- 前端通过 Axios 发起请求，统一封装在 `ruoyi-ui/src/api/` 目录
- 后端统一返回格式: `AjaxResult` (code, msg, data)
- 分页数据使用 `TableDataInfo` 包装，集成 PageHelper 分页插件
- 前端基础控制器继承 `BaseController` 获取分页参数和通用方法

## 开发注意事项

### 后端开发规范

- 所有 Controller 继承 `com.ruoyi.common.core.controller.BaseController` 获取分页、用户信息等通用方法
- 使用 `AjaxResult` 返回统一格式: `AjaxResult.success()` 或 `AjaxResult.error()`
- MyBatis Mapper XML 文件放在对应模块的 `resources/mapper/` 目录下
- 新增业务模块时，domain 实体放在对应模块的 `domain` 包，service/mapper 按规范组织

### 前端开发规范

- API 调用统一封装在 `src/api/` 对应模块文件中
- 新增页面放在 `src/views/` 对应模块目录下
- 使用 Vuex 管理全局状态（用户信息、权限、字典等）
- 路由配置支持动态加载（基于后端返回的菜单权限）

### 环境要求

- JDK 1.8
- MySQL 5.7+ 或 8.0+
- Redis 3.0+
- Maven 3.0+
- Node.js >= 8.9

### 常见配置修改

**修改后端端口:**
编辑 [application.yml](ruoyi-admin/src/main/resources/application.yml) 中的 `server.port`

**修改数据库连接:**
编辑 [application-druid.yml](ruoyi-admin/src/main/resources/application-druid.yml) 中的数据源配置

**修改 Redis 连接:**
编辑 [application.yml](ruoyi-admin/src/main/resources/application.yml) 中的 `spring.redis` 配置

**修改文件上传路径:**
编辑 [application.yml](ruoyi-admin/src/main/resources/application.yml) 中的 `ruoyi.profile` 配置

**修改前端 API 代理地址:**
编辑 [vue.config.js](ruoyi-ui/vue.config.js) 中的 `baseUrl` 变量（默认 `http://localhost:8080`）

## 新增功能开发流程 (强制规范)

当用户要求新增一个功能时，必须按照以下步骤进行引导：

1. **模块归属确认**: 主动询问用户是在现有的功能模块（如 `ruoyi-system`）上增加功能，还是需要按照“新增独立业务模块规范”新起一个模块。
2. **需求细节获取**: 要求用户提供该功能的具体需求描述（无论简单需求还是复杂业务逻辑）。
3. **适用性评估**:
   - 评估新增功能是否与 RuoYi-Vue 现有的 CRUD 风格、技术栈或业务模式一致。
   - 如果新功能与系统现有逻辑差异巨大（例如：涉及完全不同的 UI 框架、非关系型数据库深度操作或非标准协议），**必须提示用户**：*“当前 RuoYi 技能可能不完全适合此特定需求，建议先进行架构评估或手动设计。”*

## 新增独立业务模块规范

当需要新增或解耦一个独立的业务模块（如 `ruoyi-library`）时，应遵循以下规范：

### 1. 后端模块结构
- **代码包名**: `com.ruoyi.{module}`，如 `com.ruoyi.library`。
- **子目录**:
  - `domain`: 实体类
  - `mapper`: MyBatis 接口
  - `service`: 业务接口及实现
  - `controller`: Web 接口
- **资源路径**:
  - Mapper XML: `src/main/resources/mapper/{module}/*.xml`
  - SQL 脚本: `src/main/resources/*.sql`（建议直接放在 resources 根目录以确保多模块类路径扫描稳定性）

### 2. Maven 配置集成
- **父项目 (`pom.xml`)**:
  - `<modules>` 增加模块名。
  - `<dependencyManagement>` 增加模块版本声明。
- **子模块 (`pom.xml`)**: 继承父项目，依赖 `ruoyi-common`。
- **Web 入口 (`ruoyi-admin/pom.xml`)**: 必须显式添加新模块的依赖。

### 3. 开发同步规范
- **请求路径**: 使用模块名作为前缀，如 `@RequestMapping("/library/book")`。
- **权限标识**: 使用模块名作为前缀，如 `library:book:list`。
- **前端资源**:
  - API 定义: `src/api/{module}/{business}.js`
  - 页面视图: `src/views/{module}/{business}/index.vue`
- **数据库同步**: 迁移或重构物理路径后，必须同步更新 `sys_menu` 表中的 `component` 和 `perms` 字段。

### 4. 常见问题
- **点击菜单无反应**: 通常是数据库中 `sys_menu.component` 路径与 Vue 文件物理路径不匹配。
- **SQL 找不到**: 检查 `application.yml` 中的 `schema-locations` 是否包含正确的类路径前缀（如 `classpath:init.sql`）。
- **字段缺失**: `CREATE TABLE IF NOT EXISTS` 不会更新已存在的表结构，新增字段需使用 `ALTER TABLE`。

## 文档和资源

- 官方文档: http://doc.ruoyi.vip
- 在线演示: http://vue.ruoyi.vip (admin/admin123)
- 仓库地址: https://gitee.com/y_project/RuoYi-Vue
