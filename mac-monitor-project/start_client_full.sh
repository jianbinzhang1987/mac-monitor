#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
SOCKET_FILE="/tmp/mac_monitor_audit.sock"

# 0. Cleanup Socket Forcefully (silent)
if [ -e "$SOCKET_FILE" ]; then
    OWNER=$(stat -f '%Su' "$SOCKET_FILE" 2>/dev/null || echo "unknown")
    if [ "$OWNER" = "root" ]; then
        sudo rm -f "$SOCKET_FILE" 2>/dev/null
    else
        rm -f "$SOCKET_FILE" 2>/dev/null
    fi
fi

# 1. Start Audit Service (Background, Silent)
export QUIET=true
if [ -f "$SCRIPT_DIR/start_audit_service.sh" ]; then
    "$SCRIPT_DIR/start_audit_service.sh"
    
    # Brief verification
    sleep 1
    if ! pgrep -f "AuditService" > /dev/null; then
        echo "❌ AuditService failed to start! Check: tail -20 /tmp/audit_service.log"
        exit 1
    fi
else
    echo "❌ Error: start_audit_service.sh not found!"
    exit 1
fi

# 2. Start Tauri GUI (Dev Mode)
cd "$SCRIPT_DIR/gui-app" || exit
npm run tauri dev
