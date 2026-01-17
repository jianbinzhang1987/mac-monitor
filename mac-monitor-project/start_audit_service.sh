#!/bin/bash

echo "🛑 Stopping any running AuditService..."
pkill -f AuditService 2>/dev/null || true

echo "🧹 Cleaning up old socket file..."
# Check if socket exists and who owns it
if [ -e /tmp/mac_monitor_audit.sock ]; then
    OWNER=$(ls -l /tmp/mac_monitor_audit.sock | awk '{print $3}')
    if [ "$OWNER" = "root" ]; then
        echo "⚠️  Socket is owned by root. You'll need to enter your password to remove it."
        sudo rm -f /tmp/mac_monitor_audit.sock
    else
        rm -f /tmp/mac_monitor_audit.sock
    fi
    echo "✅ Old socket removed"
fi

echo "🚀 Starting AuditService..."
cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin

# Start the service in the background
./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 &
SERVICE_PID=$!

echo "⏳ Waiting for service to initialize..."
sleep 3

# Check if the process is still running
if ! ps -p $SERVICE_PID > /dev/null; then
    echo "❌ AuditService failed to start. Check the log:"
    cat /tmp/audit_service.log
    exit 1
fi

# Check if socket was created
if [ ! -e /tmp/mac_monitor_audit.sock ]; then
    echo "❌ Socket file not created. Check the log:"
    cat /tmp/audit_service.log
    exit 1
fi

echo "✅ AuditService is running (PID: $SERVICE_PID)"
echo "✅ Socket created: $(ls -la /tmp/mac_monitor_audit.sock)"
echo ""
echo "🎉 AuditService is ready!"
echo "📝 Monitor logs with: tail -f /tmp/audit_service.log"
echo "📝 Service logs also at: /tmp/mac_monitor_audit_service.log"
