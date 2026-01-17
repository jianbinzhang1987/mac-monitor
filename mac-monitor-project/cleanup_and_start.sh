#!/bin/bash
set -e

echo "======================================"
echo "  AuditService Cleanup & Restart"
echo "======================================"
echo ""

# Stop any running instances
echo "🛑 Stopping AuditService..."
pkill -f AuditService 2>/dev/null || true
sleep 1

# Check if socket exists and is owned by root
if [ -e /tmp/mac_monitor_audit.sock ]; then
    OWNER=$(ls -l /tmp/mac_monitor_audit.sock | awk '{print $3}')
    echo "📍 Found existing socket owned by: $OWNER"

    if [ "$OWNER" = "root" ]; then
        echo "🔑 Socket is owned by root - removing with sudo..."
        sudo rm -f /tmp/mac_monitor_audit.sock
        echo "✅ Root-owned socket removed"
    else
        echo "🧹 Removing socket..."
        rm -f /tmp/mac_monitor_audit.sock
        echo "✅ Socket removed"
    fi
else
    echo "✅ No existing socket found"
fi

# Start the service
echo ""
echo "🚀 Starting AuditService..."
cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin
./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 &
SERVICE_PID=$!

echo "⏳ Waiting for initialization (3 seconds)..."
sleep 3

# Verify the process is running
if ! ps -p $SERVICE_PID > /dev/null 2>&1; then
    echo ""
    echo "❌ ERROR: AuditService failed to start!"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Last 20 lines of log:"
    tail -20 /tmp/audit_service.log
    exit 1
fi

# Verify socket was created
if [ ! -e /tmp/mac_monitor_audit.sock ]; then
    echo ""
    echo "❌ ERROR: Socket file was not created!"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Last 20 lines of log:"
    tail -20 /tmp/audit_service.log
    exit 1
fi

# Check if the socket is actually listening
if ! lsof /tmp/mac_monitor_audit.sock > /dev/null 2>&1; then
    echo ""
    echo "⚠️  WARNING: Socket exists but no process is listening on it!"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Last 30 lines of log:"
    tail -30 /tmp/audit_service.log
    echo ""
    echo "The service may have panicked during IPC initialization."
    echo "Check /tmp/mac_monitor_audit_service.log for more details."
    exit 1
fi

echo ""
echo "✅ AuditService is running!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  PID: $SERVICE_PID"
echo "  Socket: $(ls -la /tmp/mac_monitor_audit.sock)"
echo "  Listening process: $(lsof /tmp/mac_monitor_audit.sock | tail -1)"
echo ""
echo "📝 Monitor logs:"
echo "  - Swift/stderr: tail -f /tmp/audit_service.log"
echo "  - Rust core: tail -f /tmp/mac_monitor_audit_service.log"
echo ""
echo "🎉 Ready to accept connections!"
