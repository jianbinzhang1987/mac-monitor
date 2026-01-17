#!/bin/bash
set -e
set -x # 启用调试输出

# ==========================================
# Mac Monitor 自动化构建脚本
# ==========================================

# 0. 环境检测与准备
# ------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export PROJECT_ROOT="$SCRIPT_DIR"

echo "📍 项目根目录: $PROJECT_ROOT"

# 清理旧的 Socket 环境 (关键修复)
SOCKET_FILE="/tmp/mac_monitor_audit.sock"
if [ -e "$SOCKET_FILE" ]; then
    echo "🧹 正在清理旧的 Socket 文件..."
    if [ -w "$SOCKET_FILE" ]; then
        rm -f "$SOCKET_FILE"
    else
        echo "⚠️  警告: Socket 文件 $SOCKET_FILE 被锁定或权限不足(通常是 root 拥有)。"
        echo "   尝试使用 sudo 清理..."
        sudo rm -f "$SOCKET_FILE" || {
            echo "❌ 无法清理 Socket 文件。请手动运行: sudo rm -f $SOCKET_FILE"
            # 不直接退出，给用户一个修复机会
        }
    fi
fi

# Detect Architecture
ARCH_NAME=$(uname -m)
if [ "$ARCH_NAME" = "x86_64" ]; then
    TARGET_TRIPLE="x86_64-apple-darwin"
    XCODE_ARCH="x86_64"
elif [ "$ARCH_NAME" = "arm64" ]; then
    TARGET_TRIPLE="aarch64-apple-darwin"
    XCODE_ARCH="arm64" 
    # 注意: 如果 Rust 是 x86_64 库但系统是 arm64，可能需要交叉编译或强制 xcodebuild 用 x86_64
    # 在本例中，我们假设 Rust 库已匹配架构或 Xcode 将强制匹配
    # 根据之前会话，Rust 库是 x86_64，我们强制 Xcode 用 x86_64
    XCODE_ARCH="x86_64"
else
    echo "❌ 不支持的架构: $ARCH_NAME"
    exit 1
fi
echo "🖥️  检测到架构: $ARCH_NAME (Rust Target: $TARGET_TRIPLE, Xcode Arch: $XCODE_ARCH)"

# 设定 Xcode 开发者目录 (避免 CLT 问题)
export DEVELOPER_DIR="/Applications/Xcode.app/Contents/Developer"

# 1. 构建 Network Extension Rust Core
# ------------------------------------------
echo ""
echo "🦀 [1/4] 构建 Network Extension Rust Core..."
cd "$PROJECT_ROOT/network-extension/rust-core"
cargo build --release
RUST_LIB_PATH="$PROJECT_ROOT/network-extension/rust-core/target/release/libnetwork_procotol_stack.a"

if [ ! -f "$RUST_LIB_PATH" ]; then
    echo "❌ Rust 静态库未找到: $RUST_LIB_PATH"
    exit 1
fi
echo "✅ Rust Core 构建完成"

# 2. 构建 Network Extension (.appex)
# ------------------------------------------
echo ""
echo "🔌 [2/4] 构建 Network Extension (.appex)..."
NE_PROJECT_DIR="$PROJECT_ROOT/mitmproxy_rs/mitmproxy-macos/redirector"
cd "$NE_PROJECT_DIR"

# 动态修复工程文件配置 (确保 Bundle ID 和 模块名 正确，移除签名限制)
PBXPROJ="$NE_PROJECT_DIR/macos-redirector.xcodeproj/project.pbxproj"
INFO_PLIST="$NE_PROJECT_DIR/network-extension/Info.plist"

# 2.0 预处理 Info.plist (移除 mitmproxy 引用)
plutil -replace NSSystemExtensionUsageDescription -string "Monitor network traffic" "$INFO_PLIST"
plutil -replace CFBundleDisplayName -string "Mac Monitor Extension" "$INFO_PLIST"
plutil -replace CFBundleName -string "Mac Monitor Extension" "$INFO_PLIST"

# 2.1 修复 pbxproj (Bundle ID, 移除 Entitlements 引用, 移除 Team ID)
# 这里的 sed 命令需谨慎，确保幂等性或能处理已修改状态
# 我们使用 xcodebuild 参数覆盖大部分设置，但 ENTITLEMENTS 引用必须移除才能免签名构建
sed -i '' '/CODE_SIGN_ENTITLEMENTS/d' "$PBXPROJ"
# sed -i '' 's/com.apple.product-type.system-extension/com.apple.product-type.app-extension/g' "$PBXPROJ"
# 替换 Bundle ID (如果还未替换)
sed -i '' 's/org.mitmproxy.macos-redirector/com.mac-monitor-gui.app/g' "$PBXPROJ"

# 注入 PRODUCT_MODULE_NAME 和 PRODUCT_NAME 到 pbxproj (针对 network-extension target)
# 查找 network-extension 的 Release 配置块中的 PRODUCT_BUNDLE_IDENTIFIER 并追加
sed -i '' '/PRODUCT_BUNDLE_IDENTIFIER = "com.mac-monitor-gui.app.network-extension";/a\
				PRODUCT_MODULE_NAME = network_extension;\
				PRODUCT_NAME = "network-extension";' "$PBXPROJ"

# 2.2 构建扩展 (Release, Unsigned, X86_64 forced)
echo "   正在清理旧构建..."
rm -rf "$NE_PROJECT_DIR/build"

echo "   正在解析依赖..."
xcodebuild -project macos-redirector.xcodeproj -resolvePackageDependencies

echo "   正在执行 xcodebuild (Target: network-extension)..."
# 使用 -jobs 4 限制并发，防止资源耗尽导致 Interrupted
xcodebuild -project macos-redirector.xcodeproj \
    -target network-extension \
    -configuration Release \
    build \
    -jobs 4 \
    CODE_SIGN_IDENTITY="" \
    CODE_SIGNING_REQUIRED=NO \
    SIGNING_ALLOWED=NO \
    DEVELOPMENT_TEAM="" \
    OTHER_LDFLAGS="-force_load $RUST_LIB_PATH -framework Security -framework NetworkExtension -framework SystemConfiguration -framework CoreFoundation" \
    -arch "$XCODE_ARCH" \
    SYMROOT="$NE_PROJECT_DIR/build" \
    > build_appex.log 2>&1 || { echo "❌ 构建失败，查看 $NE_PROJECT_DIR/build_appex.log"; cat build_appex.log; exit 1; }

# 定位产物
APPEX_PATH=$(find "$NE_PROJECT_DIR/build" -name "*.appex" | head -n 1)
if [ -n "$APPEX_PATH" ]; then
    # 如果产物名为 .appex (因 PRODUCT_NAME 缺失), 重命名为 network-extension.appex
    if [ "$(basename "$APPEX_PATH")" == ".appex" ]; then
        echo "⚠️  发现隐形文件 .appex, 正在重命名..."
        mv "$APPEX_PATH" "$(dirname "$APPEX_PATH")/network-extension.appex"
        APPEX_PATH="$(dirname "$APPEX_PATH")/network-extension.appex"
    fi
    
    # --- 修复结构与元数据 ---
    echo "   正在修复 .appex 内部结构与元数据..."
    NE_INFO_PLIST="$APPEX_PATH/Contents/Info.plist"
    
    # 1. 确保 CFBundleExecutable 存在
    plutil -replace CFBundleExecutable -string "network-extension" "$NE_INFO_PLIST"
    
    # 2. 确保 MacOS 是目录并包含 executable
    if [ -f "$APPEX_PATH/Contents/MacOS" ]; then
        echo "   ⚠️  检测到 MacOS 为文件, 正在转换为目录..."
        mv "$APPEX_PATH/Contents/MacOS" "$APPEX_PATH/Contents/network-extension"
        mkdir -p "$APPEX_PATH/Contents/MacOS"
        mv "$APPEX_PATH/Contents/network-extension" "$APPEX_PATH/Contents/MacOS/network-extension"
        chmod +x "$APPEX_PATH/Contents/MacOS/network-extension"
    fi
fi

if [ -z "$APPEX_PATH" ]; then
    echo "❌ 错误: 未找到生成的 .appex 文件"
    # 尝试查找 .systemextension 作为备用 (并重命名)
    SYSEX_PATH=$(find "$NE_PROJECT_DIR/build" -name "*.systemextension" | head -n 1)
    if [ -n "$SYSEX_PATH" ]; then
        echo "⚠️  发现 .systemextension, 将尝试作为 .appex 使用..."
        APPEX_PATH="${SYSEX_PATH%.systemextension}.appex"
        mv "$SYSEX_PATH" "$APPEX_PATH"
    else
        exit 1
    fi
fi
echo "✅ Network Extension 构建成功: $APPEX_PATH"


# 3. 构建 AuditService (Sidecar as .app bundle)
# ------------------------------------------
echo ""
echo "🛡️  [3/4] 构建 AuditService (Sidecar)..."

# 3.1 构建 AuditService Rust Core
echo "   🦀 Building AuditService Rust Core..."
cd "$PROJECT_ROOT/audit-service/rust-core"
cargo build --release
if [ ! -f "target/release/libaudit_logic_core.a" ]; then
    echo "❌ AuditService Rust Core 构建失败"
    exit 1
fi

# 3.2 构建 AuditService Swift
echo "   Swift Building..."
cd "$PROJECT_ROOT/audit-service/swift"
swift build -c release
echo "✅ AuditService 编译完成"

# 准备 Sidecar 目录
SIDECAR_DIR="$PROJECT_ROOT/gui-app/src-tauri/bin"
mkdir -p "$SIDECAR_DIR"

# 查找二进制文件
SOURCE_BIN="$PROJECT_ROOT/audit-service/swift/.build/release/AuditService"
if [ ! -f "$SOURCE_BIN" ]; then
    # 尝试查找特定架构下的构建
    SOURCE_BIN=$(find .build -name AuditService -type f | grep release | head -n 1)
fi

# 创建 .app bundle 结构 (用于录屏权限识别)
echo "📦 正在创建 AuditService.app bundle..."
APP_BUNDLE="$SIDECAR_DIR/AuditService.app"
rm -rf "$APP_BUNDLE"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# 复制二进制文件
cp "$SOURCE_BIN" "$APP_BUNDLE/Contents/MacOS/AuditService"
chmod +x "$APP_BUNDLE/Contents/MacOS/AuditService"

# 创建 Info.plist
cat > "$APP_BUNDLE/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleDevelopmentRegion</key>
	<string>en</string>
	<key>CFBundleExecutable</key>
	<string>AuditService</string>
	<key>CFBundleIdentifier</key>
	<string>com.mac-monitor.audit-service</string>
	<key>CFBundleInfoDictionaryVersion</key>
	<string>6.0</string>
	<key>CFBundleName</key>
	<string>Mac Monitor Audit Service</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>CFBundleShortVersionString</key>
	<string>1.0</string>
	<key>CFBundleVersion</key>
	<string>1</string>
	<key>LSMinimumSystemVersion</key>
	<string>12.3</string>
	<key>NSHighResolutionCapable</key>
	<true/>
	<key>NSScreenCaptureUsageDescription</key>
	<string>Mac Monitor 需要录屏权限来监控终端活动、捕获屏幕内容并进行 OCR 文字识别，以实现安全审计功能。</string>
	<key>NSSystemAdministrationUsageDescription</key>
	<string>Mac Monitor 需要系统管理权限来监控进程活动和网络连接。</string>
	<key>LSUIElement</key>
	<true/>
</dict>
</plist>
EOF

# Ad-hoc 签名 (确保系统识别)
codesign --force --deep --sign - "$APP_BUNDLE" 2>/dev/null || echo "⚠️  签名失败，继续..."

echo "✅ AuditService.app bundle 创建成功"

# 同时保留旧的命名方式作为 sidecar (用于兼容性)
TARGET_BIN="$SIDECAR_DIR/AuditService-$TARGET_TRIPLE"
cp "$SOURCE_BIN" "$TARGET_BIN"
chmod +x "$TARGET_BIN"
echo "✅ Sidecar 部署成功"

# 3.5 构建 VPN 辅助工具 (Sidecar)
echo ""
echo "🔌 [3.5/4] 构建 VPN 辅助工具 (Sidecar)..."
cd "$PROJECT_ROOT/gui-app/src-tauri/swift"
swiftc -o vpn-helper main.swift VPNManager.swift ProxyManager.swift -framework NetworkExtension -framework Foundation
TARGET_VPN_BIN="$SIDECAR_DIR/vpn-helper-$TARGET_TRIPLE"
cp vpn-helper "$TARGET_VPN_BIN"
chmod +x "$TARGET_VPN_BIN"
echo "✅ VPN Helper 部署成功"

# 3.6 构建 HTTP 代理服务 (Sidecar)
echo ""
echo "🌐 [3.6/4] 构建 HTTP 代理服务 (Sidecar)..."
cd "$PROJECT_ROOT/traffic-proxy"
cargo build --release
PROXY_BIN="$PROJECT_ROOT/traffic-proxy/target/release/traffic-proxy"
TARGET_PROXY_BIN="$SIDECAR_DIR/traffic-proxy-$TARGET_TRIPLE"
if [ -f "$PROXY_BIN" ]; then
    cp "$PROXY_BIN" "$TARGET_PROXY_BIN"
    chmod +x "$TARGET_PROXY_BIN"
    echo "✅ Traffic Proxy 部署成功"
else
    echo "❌ Traffic Proxy 构建失败"
    exit 1
fi


# 4. 构建 Tauri GUI 应用 & 集成扩展
# ------------------------------------------
echo ""
echo "🖥️  [4/4] 构建 Tauri GUI 应用..."
cd "$PROJECT_ROOT/gui-app"

if [ ! -d "node_modules" ]; then
    npm install
fi

echo "🚀 开始 Tauri 构建..."
echo "🚀 开始 Tauri 构建..."
npm run tauri build > tauri_build.log 2>&1 || { echo "❌ Tauri 构建失败"; cat tauri_build.log; exit 1; }

# 嵌入 Network Extension
APP_BUNDLE_PATH="$PROJECT_ROOT/gui-app/src-tauri/target/release/bundle/macos/Mac Monitor.app"
PLUGINS_DIR="$APP_BUNDLE_PATH/Contents/PlugIns"

echo "🧩 正在嵌入 Network Extension..."
mkdir -p "$PLUGINS_DIR"
cp -r "$APPEX_PATH" "$PLUGINS_DIR/"

echo ""
echo "💿 DMG 安装包位置: output/Mac Monitor.dmg"

# Generate DMG using appdmg (npx) for correct layout
echo "📀 正在生成 DMG (修复图标重叠)..."
if command -v npm >/dev/null; then
    rm -f "$PROJECT_ROOT/output/Mac Monitor.dmg"
    npx -y appdmg "$PROJECT_ROOT/dmg-config.json" "$PROJECT_ROOT/output/Mac Monitor.dmg" || echo "⚠️ appdmg 生成失败"
else
    echo "⚠️ npm 未安装，跳过 DMG 生成"
fi

# Copy App to output
mkdir -p "$PROJECT_ROOT/output"
# cp -r "$APP_BUNDLE_PATH" "$PROJECT_ROOT/output/"

# Cleanup previous dmg copy attemp if any (we now generate directly to output)
# cp "${APP_BUNDLE_PATH%/*/*}/dmg/"*.dmg "$PROJECT_ROOT/output/" 2>/dev/null || true

echo "📦 已发布到 output/ 目录下"

echo "⚠️  注意: 由于是无签名/Ad-hoc构建，Network Extension 可能需要关闭 SIP 或手动签名才能加载。"
echo "🎉 ========================================"
