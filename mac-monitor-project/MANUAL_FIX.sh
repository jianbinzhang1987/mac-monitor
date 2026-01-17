#!/bin/bash
# Manual cleanup script - Run this in your terminal to fix the socket issue

echo "======================================"
echo "  Manual AuditService Fix"
echo "======================================"
echo ""
echo "The socket file is owned by root and needs to be removed manually."
echo "Please run the following commands in your terminal:"
echo ""
echo "1. Stop any running AuditService:"
echo "   pkill -f AuditService"
echo ""
echo "2. Remove the root-owned socket (will ask for password):"
echo "   sudo rm -f /tmp/mac_monitor_audit.sock"
echo ""
echo "3. Start AuditService fresh:"
echo "   cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin"
echo "   ./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 &"
echo ""
echo "4. Verify it's running (after 3 seconds):"
echo "   sleep 3 && lsof /tmp/mac_monitor_audit.sock"
echo ""
echo "5. You should see the AuditService process listening on the socket."
echo ""

# Create a one-liner version
echo "======================================"
echo "  Or run this single command:"
echo "======================================"
echo ""
cat << 'EOF'
pkill -f AuditService; sudo rm -f /tmp/mac_monitor_audit.sock && cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin && ./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 & sleep 3 && echo "Service started!" && lsof /tmp/mac_monitor_audit.sock
EOF
echo ""
