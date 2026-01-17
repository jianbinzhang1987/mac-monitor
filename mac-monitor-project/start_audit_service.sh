#!/bin/bash

# Silent mode for production use
QUIET=${QUIET:-false}

log() {
    if [ "$QUIET" != "true" ]; then
        echo "$1"
    fi
}

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

log "🛑 Stopping any running AuditService..."
pkill -f AuditService 2>/dev/null || true

log "🧹 Cleaning up old socket file..."
if [ -e /tmp/mac_monitor_audit.sock ]; then
    OWNER=$(stat -f '%Su' /tmp/mac_monitor_audit.sock 2>/dev/null || echo "unknown")
    if [ "$OWNER" = "root" ]; then
        log "⚠️  Socket is owned by root. Removing with sudo..."
        sudo rm -f /tmp/mac_monitor_audit.sock 2>/dev/null
    else
        rm -f /tmp/mac_monitor_audit.sock
    fi
fi

log "🚀 Starting AuditService..."
cd "$SCRIPT_DIR/gui-app/src-tauri/bin"

# Start the service in the background, redirect ALL output to log file
nohup ./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 &
SERVICE_PID=$!

# Disown the process so it continues running after shell exits
disown $SERVICE_PID 2>/dev/null

log "⏳ Waiting for service to initialize..."
sleep 2

# Check if the process is still running
if ! ps -p $SERVICE_PID > /dev/null 2>&1; then
    echo "❌ AuditService failed to start. Check: tail -20 /tmp/audit_service.log"
    exit 1
fi

# Check if socket was created
if [ ! -e /tmp/mac_monitor_audit.sock ]; then
    echo "❌ Socket file not created. Check: tail -20 /tmp/audit_service.log"
    exit 1
fi

log "✅ AuditService is running (PID: $SERVICE_PID)"
