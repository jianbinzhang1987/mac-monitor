#!/bin/bash
# Quick fix script - removes root-owned socket and restarts AuditService

echo "🔧 Quick Fix for AuditService Connection Issue"
echo "=============================================="
echo ""

# Stop any running instances
echo "1️⃣  Stopping AuditService..."
pkill -f AuditService 2>/dev/null || echo "   No running instance found"

# Remove the socket (will ask for password if owned by root)
echo ""
echo "2️⃣  Removing old socket..."
if [ -e /tmp/mac_monitor_audit.sock ]; then
    OWNER=$(ls -l /tmp/mac_monitor_audit.sock | awk '{print $3}')
    if [ "$OWNER" = "root" ]; then
        echo "   ⚠️  Socket is owned by root - using sudo..."
        sudo rm -f /tmp/mac_monitor_audit.sock
        echo "   ✅ Removed"
    else
        rm -f /tmp/mac_monitor_audit.sock
        echo "   ✅ Removed"
    fi
else
    echo "   ✅ No old socket found"
fi

# Start AuditService
echo ""
echo "3️⃣  Starting AuditService..."
cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin
./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 &
SERVICE_PID=$!
echo "   Started with PID: $SERVICE_PID"

# Wait for initialization
echo ""
echo "4️⃣  Waiting for initialization (3 seconds)..."
sleep 3

# Verify
echo ""
echo "5️⃣  Verifying service..."

if ! ps -p $SERVICE_PID > /dev/null 2>&1; then
    echo "   ❌ ERROR: Service crashed during startup!"
    echo ""
    echo "   Last 30 lines of log:"
    tail -30 /tmp/audit_service.log
    exit 1
fi

if [ ! -e /tmp/mac_monitor_audit.sock ]; then
    echo "   ❌ ERROR: Socket not created!"
    echo ""
    echo "   Service is running but socket wasn't created. Check logs:"
    tail -30 /tmp/audit_service.log
    exit 1
fi

if lsof /tmp/mac_monitor_audit.sock > /dev/null 2>&1; then
    echo "   ✅ Service is running: PID $SERVICE_PID"
    echo "   ✅ Socket created: /tmp/mac_monitor_audit.sock"
    echo "   ✅ Socket is listening: $(lsof /tmp/mac_monitor_audit.sock | tail -1 | awk '{print $1}')"
else
    echo "   ⚠️  WARNING: Socket exists but nothing is listening!"
    echo ""
    echo "   This means the IPC server failed to bind. Check logs:"
    tail -30 /tmp/audit_service.log | grep -E "(panic|error|IPC|socket)"
    exit 1
fi

echo ""
echo "✅ SUCCESS! AuditService is ready."
echo ""
echo "📝 Monitor logs with:"
echo "   tail -f /tmp/audit_service.log"
echo "   tail -f /tmp/mac_monitor_audit_service.log"
echo ""
echo "🧪 Test connection with:"
echo "   echo '{\"command\":\"get_pops\",\"payload\":null}' | nc -U /tmp/mac_monitor_audit.sock"
