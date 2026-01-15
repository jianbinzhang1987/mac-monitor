#!/bin/bash
set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo "🔧 [1/2] Building Rust Core..."
cd rust-core
# Create dummy db for sqlx if not exists (though we fixed schema, this is double safety)
if [ ! -f "audit.db" ]; then
    touch audit.db
fi
export DATABASE_URL=sqlite:audit.db
cargo build --release

echo "🔧 [2/2] Building Swift Client..."
cd ../swift

# Clean potential locks
echo "   Cleaning build artifacts & Releasing locks..."
pkill swift-build || true
rm -rf .build

echo "   Compiling..."
swift build

echo "✅ Build Complete! You can now run ./run_client.sh"
