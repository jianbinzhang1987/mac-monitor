#!/bin/bash
echo "🔍 Starting AuditService Debug Mode..."

# 1. Cleanup Socket
if [ -e /tmp/mac_monitor_audit.sock ]; then
    echo "🧹 Removing old socket (needs sudo)..."
    sudo rm -f /tmp/mac_monitor_audit.sock
fi

# 2. Run Service in Foreground
echo "🚀 Running AuditService in foreground..."
echo "----------------------------------------"
cd "$(dirname "$0")/gui-app/src-tauri/bin"
./AuditService.app/Contents/MacOS/AuditService
echo "----------------------------------------"
echo "❌ Service exited with code: $?"
