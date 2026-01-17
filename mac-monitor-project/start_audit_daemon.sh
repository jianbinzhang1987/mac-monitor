#!/bin/bash
# 持久化启动 AuditService 的脚本

set -e

AUDIT_BIN="/Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin/AuditService.app/Contents/MacOS/AuditService"
LOG_FILE="/tmp/audit_service.log"
PID_FILE="/tmp/audit_service.pid"

echo "🚀 Starting AuditService Daemon"
echo "================================"

# 停止旧进程
if [ -f "$PID_FILE" ]; then
    OLD_PID=$(cat "$PID_FILE")
    if ps -p $OLD_PID > /dev/null 2>&1; then
        echo "🛑 Stopping old instance (PID: $OLD_PID)..."
        kill $OLD_PID 2>/dev/null || true
        sleep 1
    fi
    rm -f "$PID_FILE"
fi

# 清理旧的 socket
echo "🧹 Cleaning up socket..."
if [ -e /tmp/mac_monitor_audit.sock ]; then
    OWNER=$(ls -l /tmp/mac_monitor_audit.sock | awk '{print $3}')
    if [ "$OWNER" = "root" ]; then
        echo "⚠️  Socket owned by root, using sudo..."
        sudo rm -f /tmp/mac_monitor_audit.sock || true
    else
        rm -f /tmp/mac_monitor_audit.sock
    fi
fi

# 启动服务（前台模式，在新终端中）
echo "🚀 Launching AuditService..."
echo "   Binary: $AUDIT_BIN"
echo "   Log: $LOG_FILE"

# 使用 osascript 在新终端窗口中启动（这样可以看到运行状态）
osascript <<EOF
tell application "Terminal"
    activate
    set newTab to do script "cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin && ./AuditService.app/Contents/MacOS/AuditService 2>&1 | tee /tmp/audit_service.log"
    set custom title of newTab to "AuditService Daemon"
end tell
EOF

echo ""
echo "✅ AuditService launched in new Terminal window"
echo ""
echo "⏳ Waiting 3 seconds for initialization..."
sleep 3

# 验证
if [ -e /tmp/mac_monitor_audit.sock ]; then
    echo "✅ Socket created: /tmp/mac_monitor_audit.sock"
    if lsof /tmp/mac_monitor_audit.sock > /dev/null 2>&1; then
        PID=$(lsof -t /tmp/mac_monitor_audit.sock)
        echo "✅ Service is listening (PID: $PID)"
        echo "$PID" > "$PID_FILE"
        echo ""
        echo "🎉 AuditService is ready!"
        echo ""
        echo "📝 View logs: tail -f /tmp/audit_service.log"
        echo "🛑 Stop service: kill $PID"
    else
        echo "❌ Socket exists but no process is listening"
        exit 1
    fi
else
    echo "❌ Socket file not created"
    echo "Check the Terminal window for error messages"
    exit 1
fi
