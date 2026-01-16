#!/bin/bash
set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

# Parse arguments
CLEAN_BUILD=false
for arg in "$@"; do
    if [ "$arg" == "--clean" ] || [ "$arg" == "-c" ]; then
        CLEAN_BUILD=true
    fi
done

echo "🔧 [1/2] Building Rust Core..."
cd rust-core
# Create dummy db for sqlx if not exists
if [ ! -f "audit.db" ]; then
    touch audit.db
fi
export DATABASE_URL=sqlite:audit.db

if [ "$CLEAN_BUILD" = true ]; then
    echo "   Performing clean build for Rust..."
    cargo clean
fi
cargo build --release

echo "🔧 [2/2] Building Swift Client..."
cd ../swift

if [ "$CLEAN_BUILD" = true ]; then
    echo "   Cleaning Swift build artifacts & Releasing locks..."
    pkill swift-build || true
    rm -rf .build
fi

echo "   Compiling..."
# Swift build is incremental by default if .build directory exists
swift build

echo "✅ Build Complete! You can now run ./run_client.sh"
