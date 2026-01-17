#!/bin/bash
# 简单的 AuditService 启动测试

echo "🧪 Testing AuditService startup..."
echo ""

AUDIT_BIN="/Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin/AuditService.app/Contents/MacOS/AuditService"

# 清理
rm -f /tmp/mac_monitor_audit.sock
pkill -f AuditService 2>/dev/null

echo "Starting AuditService in foreground..."
echo "Press Ctrl+C to stop"
echo ""

# 直接运行（前台）
exec "$AUDIT_BIN"
