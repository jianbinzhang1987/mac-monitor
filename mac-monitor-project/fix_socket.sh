#!/bin/bash
# This script removes the stale socket file owned by root
# Run it manually with: bash fix_socket.sh

echo "🧹 Removing stale socket file (requires sudo)..."
sudo rm -f /tmp/mac_monitor_audit.sock

echo "✅ Socket removed. You can now start the AuditService."
echo "Run: /Users/adolf/Desktop/code/clash/mac-monitor-project/start_audit_service.sh"
