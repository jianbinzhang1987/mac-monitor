# Mac Monitor Project 开发指南

本项目是一个基于 macOS System Extension 的终端审计系统，采用混合架构开发。

## 🛠 技术栈

- **GUI**: Tauri v2 + Vue 3 (TypeScript)
- **网络核心**: NetworkExtension (Swift) + Rust (smoltcp, rustls)
- **审计核心**: XPC Service (Swift) + Rust (SQLite, ScreenCaptureKit, EndpointSecurity)

## 📋 环境要求

1. **macOS**: Sonoma (14.0) 或更高版本 (需支持 ScreenCaptureKit)
2. **Xcode**: 15.0+
3. **Rust**: 最新 stable 版本 (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
4. **Node.js**: v18+ & pnpm
5. **Tauri CLI**: `cargo install tauri-cli --version "^2.0.0-beta"`

## 🚀 构建步骤

### 1. 构建 Rust 核心库

项目包含两个 Rust 核心模块，需要编译为静态库 (`.a`) 供 Swift 调用。

```bash
# 1.1 构建网络扩展核心
cd mac-monitor-project/network-extension/rust-core
# 生成 libnetwork_protocol_stack.a
cargo build --release
# 注意：你需要手动生成 C 头文件 (可以使用 cbindgen) 或使用现有的 FFI 定义

# 1.2 构建审计服务核心
cd ../../audit-service/rust-core
# 生成 libaudit_logic_core.a
cargo build --release
```

### 2. 配置 Xcode 项目

由于涉及 System Extension，建议使用 Xcode 管理签名和 entitlements。

1. 打开 `mac-monitor-project/gui-app/src-tauri/ios` (如果 Tauri 生成了 iOS/macOS 混合项目) 或直接在 Xcode 中创建 Workspace。
2. **NetworkExtension Target**:
   - 链接 `libnetwork_protocol_stack.a`
   - 添加库依赖: `SystemConfiguration.framework`, `NetworkExtension.framework`
   - **Capabilities**: Network Extensions (Packet Tunnel), App Groups
3. **AuditService Target**:
   - 链接 `libaudit_logic_core.a`
   - 添加库依赖: `ScreenCaptureKit.framework`, `Vision.framework`, `EndpointSecurity.framework`
   - **Capabilities**: Endpoint Security, App Groups

### 3. 运行 GUI 应用

```bash
cd mac-monitor-project/gui-app
pnpm install
pnpm tauri dev
```

## 🔐 签名与权限 (重要)

本项目使用了敏感的系统权限，必须正确配置 Entitlements 和 Provisioning Profile 才能运行。

### Network Extension
需要在 Apple Developer Portal 创建 Identifier 并启用 **Network Extensions** 能力。
Entitlements 文件需包含:
```xml
<key>com.apple.developer.networking.networkextension</key>
<array>
    <string>packet-tunnel-provider</string>
</array>
```

### Endpoint Security
**注意**: Endpoint Security 权限需要向 Apple 额外申请 (https://developer.apple.com/contact/request/system-extension/)。
开发阶段如果无法申请，需关闭 SIP (System Integrity Protection) 才能加载未经签名的 ES 扩展（不推荐用于生产）。

Entitlements 文件需包含:
```xml
<key>com.apple.developer.endpoint-security.client</key>
<true/>
```

### Screen Capture
需要申请屏幕录制权限。在 `Info.plist` 中添加 `NSDesktopFolderUsageDescription` 等隐私描述。

## 📝 调试

- **GUI 日志**: 浏览器控制台
- **系统扩展日志**: 使用 `Console.app` 筛选子系统 `com.macmonitor` 或使用 `log stream` 命令。

```bash
log stream --predicate 'subsystem contains "com.macmonitor"' --level debug
```
