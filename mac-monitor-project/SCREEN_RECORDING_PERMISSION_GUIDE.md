# 录屏权限配置指南

## 问题解决方案

你之前遇到的问题是:**安装包请求录屏权限时,在系统设置中看不到应用程序**。

### 根本原因
原来的 `AuditService` 是一个**裸二进制文件**,没有 `Info.plist` 文件,因此 macOS 无法识别它并在"系统设置 > 隐私与安全性 > 屏幕录制"中显示。

### 解决方案
现在 `AuditService` 被打包成完整的 **`.app` bundle**,包含:
- `Contents/MacOS/AuditService` - 可执行文件
- `Contents/Info.plist` - 包含权限描述和应用元数据
- 正确的 Bundle Identifier: `com.mac-monitor.audit-service`
- 录屏权限描述: "Mac Monitor 需要录屏权限来监控终端活动、捕获屏幕内容并进行 OCR 文字识别,以实现安全审计功能。"

## 验证步骤

### 1. 检查构建产物

```bash
# 验证 AuditService.app 已正确嵌入
ls -la "/Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/target/release/bundle/macos/Mac Monitor.app/Contents/Resources/bin/AuditService.app"

# 检查 Info.plist
plutil -p "/Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/target/release/bundle/macos/Mac Monitor.app/Contents/Resources/bin/AuditService.app/Contents/Info.plist" | grep NSScreenCaptureUsageDescription
```

### 2. 测试权限请求

#### 方法 1: 运行 Mac Monitor 应用
```bash
open "/Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/target/release/bundle/macos/Mac Monitor.app"
```

当应用启动 AuditService 时,系统会弹出权限请求对话框。

#### 方法 2: 直接运行 AuditService.app
```bash
open "/Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/target/release/bundle/macos/Mac Monitor.app/Contents/Resources/bin/AuditService.app"
```

### 3. 在系统设置中检查

1. 打开 **系统设置 (System Settings)**
2. 进入 **隐私与安全性 (Privacy & Security)**
3. 点击 **屏幕录制 (Screen Recording)**
4. 现在应该能看到 **"Mac Monitor Audit Service"** 或 **"AuditService"**

### 4. 如果看不到应用

如果在系统设置中仍然看不到应用,尝试以下步骤:

```bash
# 1. 重置 TCC 数据库 (需要重启)
tccutil reset ScreenCapture

# 2. 重新启动应用触发权限请求
open "/Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/target/release/bundle/macos/Mac Monitor.app/Contents/Resources/bin/AuditService.app"

# 3. 检查系统日志
log stream --predicate 'subsystem == "com.apple.TCC"' --level debug
```

## 构建脚本修改

`build_all.sh` 第 3 步现在会:

1. ✅ 编译 Swift 二进制文件
2. ✅ 创建 `.app` bundle 结构
3. ✅ 生成包含权限描述的 `Info.plist`
4. ✅ Ad-hoc 签名 bundle
5. ✅ 复制到 `gui-app/src-tauri/bin/`
6. ✅ Tauri 打包时将其嵌入到 `Contents/Resources/bin/`

## 关键文件位置

- **构建脚本**: `build_all.sh` (第 146-219 行)
- **AuditService.app 位置**: `gui-app/src-tauri/bin/AuditService.app`
- **最终打包位置**: `Mac Monitor.app/Contents/Resources/bin/AuditService.app`
- **Tauri 配置**: `gui-app/src-tauri/tauri.conf.json` (resources 部分)

## 权限描述内容

```xml
<key>NSScreenCaptureUsageDescription</key>
<string>Mac Monitor 需要录屏权限来监控终端活动、捕获屏幕内容并进行 OCR 文字识别,以实现安全审计功能。</string>

<key>NSSystemAdministrationUsageDescription</key>
<string>Mac Monitor 需要系统管理权限来监控进程活动和网络连接。</string>
```

## 下次构建

运行完整构建脚本:
```bash
cd /Users/adolf/Desktop/code/clash/mac-monitor-project
./build_all.sh
```

构建产物位置:
- **应用程序**: `gui-app/src-tauri/target/release/bundle/macos/Mac Monitor.app`
- **DMG 安装包**: `output/Mac Monitor.dmg`

## 注意事项

⚠️ **Ad-hoc 签名限制**: 当前使用 ad-hoc 签名 (`codesign --sign -`),适合开发测试。如果需要分发给其他用户,需要使用有效的开发者证书签名。

⚠️ **SIP (System Integrity Protection)**: Network Extension 部分可能需要关闭 SIP 或使用正式签名才能加载。

✅ **录屏权限**: AuditService.app 现在应该能正常请求并在系统设置中显示。
