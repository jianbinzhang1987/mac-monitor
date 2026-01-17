#!/bin/bash
set -e

echo "🛑 Stopping AuditService..."
pkill -f AuditService || echo "No running AuditService found"

echo "🧹 Cleaning up socket file..."
# Remove the socket file (requires sudo if owned by root)
if [ -e /tmp/mac_monitor_audit.sock ]; then
    sudo rm -f /tmp/mac_monitor_audit.sock
    echo "✅ Socket file removed"
fi

echo "🚀 Starting AuditService..."
cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin
./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 &

echo "⏳ Waiting for service to initialize..."
sleep 3

echo "📋 Checking service status..."
if ps aux | grep -v grep | grep AuditService > /dev/null; then
    echo "✅ AuditService is running (PID: $(pgrep -f AuditService))"
else
    echo "❌ AuditService failed to start"
    echo "Log output:"
    cat /tmp/audit_service.log
    exit 1
fi

if [ -e /tmp/mac_monitor_audit.sock ]; then
    echo "✅ Socket file created: $(ls -la /tmp/mac_monitor_audit.sock)"

    # Fix permissions so GUI can connect
    sudo chmod 777 /tmp/mac_monitor_audit.sock
    echo "✅ Socket permissions fixed"
else
    echo "❌ Socket file not created"
    echo "Log output:"
    cat /tmp/audit_service.log
    exit 1
fi

echo ""
echo "🎉 AuditService is ready!"
echo "📝 Service log: tail -f /tmp/audit_service.log"
