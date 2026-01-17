#!/bin/bash
set -e

PROJECT_ROOT="/Users/adolf/Desktop/code/clash/mac-monitor-project"
cd "$PROJECT_ROOT/audit-service/swift"

echo "🦀 Building Rust Core..."
cd "$PROJECT_ROOT/audit-service/rust-core"
cargo build --release

echo "🔨 Rebuilding AuditService..."
cd "$PROJECT_ROOT/audit-service/swift"
echo "   Cleaning Swift cache to force relink..."
swift package clean
swift build -c release

echo "📦 Updating AuditService binary..."
SOURCE_BIN="$PROJECT_ROOT/audit-service/swift/.build/release/AuditService"
TARGET_APP="$PROJECT_ROOT/gui-app/src-tauri/bin/AuditService.app/Contents/MacOS/AuditService"

cp "$SOURCE_BIN" "$TARGET_APP"
chmod +x "$TARGET_APP"

echo "✅ AuditService updated."
