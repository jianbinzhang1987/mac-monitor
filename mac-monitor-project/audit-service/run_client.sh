#!/bin/bash
set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR/swift"

echo "🛡 Starting Audit Service..."
./.build/debug/AuditService
