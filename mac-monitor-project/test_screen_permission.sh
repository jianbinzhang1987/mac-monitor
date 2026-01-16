#!/bin/bash
# 测试脚本：验证 AuditService.app 的录屏权限请求

echo "🧪 测试录屏权限请求"
echo "===================="
echo ""

APP_PATH="/Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/target/release/bundle/macos/Mac Monitor.app/Contents/Resources/bin/AuditService.app"

# 1. 检查 app bundle 是否存在
if [ ! -d "$APP_PATH" ]; then
    echo "❌ AuditService.app 不存在: $APP_PATH"
    exit 1
fi
echo "✅ AuditService.app 存在"

# 2. 检查 Info.plist
INFO_PLIST="$APP_PATH/Contents/Info.plist"
if [ ! -f "$INFO_PLIST" ]; then
    echo "❌ Info.plist 不存在"
    exit 1
fi
echo "✅ Info.plist 存在"

# 3. 检查权限描述
if plutil -p "$INFO_PLIST" | grep -q "NSScreenCaptureUsageDescription"; then
    echo "✅ NSScreenCaptureUsageDescription 已设置:"
    plutil -p "$INFO_PLIST" | grep -A 1 "NSScreenCaptureUsageDescription"
else
    echo "❌ NSScreenCaptureUsageDescription 未设置"
    exit 1
fi

# 4. 检查 Bundle ID
BUNDLE_ID=$(plutil -p "$INFO_PLIST" | grep CFBundleIdentifier | awk -F'"' '{print $4}')
echo "📦 Bundle ID: $BUNDLE_ID"

# 5. 检查代码签名
echo ""
echo "🔐 代码签名状态:"
codesign -dv --verbose=2 "$APP_PATH" 2>&1 | grep -E "(Identifier|Signature)"

# 6. 重置权限（清除之前的拒绝记录）
echo ""
echo "🔄 重置录屏权限..."
tccutil reset ScreenCapture "$BUNDLE_ID" 2>&1

# 7. 杀死旧进程
echo ""
echo "🛑 停止旧的 AuditService 进程..."
pkill -9 AuditService 2>/dev/null
sleep 1

# 8. 启动应用并监控日志
echo ""
echo "🚀 启动 AuditService.app..."
echo "   (应该会弹出录屏权限请求对话框)"
echo ""

# 使用 open 命令启动（这样 macOS 会正确处理 app bundle）
open "$APP_PATH"

echo "⏳ 等待 5 秒，查看是否弹出权限对话框..."
sleep 5

# 9. 检查进程是否在运行
echo ""
echo "📊 进程状态:"
if ps aux | grep -v grep | grep AuditService > /dev/null; then
    echo "✅ AuditService 正在运行"
    ps aux | grep -v grep | grep AuditService | head -1
else
    echo "⚠️ AuditService 未运行（可能已退出）"
fi

# 10. 检查系统日志
echo ""
echo "📋 最近的系统日志 (TCC):"
log show --predicate 'subsystem == "com.apple.TCC"' --last 1m --style compact 2>/dev/null | grep -i "screen\|audit" | tail -10

echo ""
echo "===================="
echo "✅ 测试完成"
echo ""
echo "📌 下一步:"
echo "   1. 检查是否弹出了权限请求对话框"
echo "   2. 打开 系统设置 > 隐私与安全性 > 屏幕录制"
echo "   3. 查看是否显示 'Mac Monitor Audit Service'"
echo ""
echo "🔧 手动打开系统设置:"
echo "   open 'x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture'"
