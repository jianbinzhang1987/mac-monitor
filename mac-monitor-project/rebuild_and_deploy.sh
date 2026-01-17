#!/bin/bash
set -e
PROJECT_ROOT="/Users/adolf/Desktop/code/clash/mac-monitor-project"
SIDECAR_DIR="$PROJECT_ROOT/gui-app/src-tauri/bin"
ARCH_NAME=$(uname -m)
if [ "$ARCH_NAME" = "x86_64" ]; then
    TARGET_TRIPLE="x86_64-apple-darwin"
    XCODE_ARCH="x86_64"
elif [ "$ARCH_NAME" = "arm64" ]; then
    TARGET_TRIPLE="aarch64-apple-darwin"
    XCODE_ARCH="x86_64" # Forcing x86_64 as per build_all.sh
fi

echo "🔧 Rebuilding Traffic Proxy..."
cd "$PROJECT_ROOT/traffic-proxy"
cargo build --release
PROXY_BIN="target/release/traffic-proxy"
TARGET_PROXY_BIN="$SIDECAR_DIR/traffic-proxy-$TARGET_TRIPLE"
if [ -f "$PROXY_BIN" ]; then
    cp "$PROXY_BIN" "$TARGET_PROXY_BIN"
    chmod +x "$TARGET_PROXY_BIN"
    echo "✅ Traffic Proxy updated."
else
    echo "❌ Traffic Proxy build failed."
    exit 1
fi

echo "🔧 Rebuilding Network Extension (Rust Core)..."
cd "$PROJECT_ROOT/network-extension/rust-core"
cargo build --release
RUST_LIB_PATH="$PROJECT_ROOT/network-extension/rust-core/target/release/libnetwork_procotol_stack.a"

if [ ! -f "$RUST_LIB_PATH" ]; then
    echo "❌ Network Extension Rust Core build failed."
    exit 1
fi
echo "✅ Network Extension Rust Core updated."

echo "⚠️  Note: Network Extension appex requires full Xcode build (via build_all.sh) if you need the extension updated."
echo "⚠️  Since you are running dev mode, checking if we need to update the NE binary used by dev..."
# Usually NE is installed in system or loaded from app bundle. In dev mode, it might be tricky.
# We'll skip appex rebuild for this quick script unless requested, as it takes longer.
# But since I modified mitm.rs which is used by NE, it really SHOULD be rebuilt.

echo "🔧 Rebuilding Network Extension Appex (Fast)..."
NE_PROJECT_DIR="$PROJECT_ROOT/mitmproxy_rs/mitmproxy-macos/redirector"
cd "$NE_PROJECT_DIR"
# Assuming pbxproj is already patched by build_all.sh previously
xcodebuild -project macos-redirector.xcodeproj \
    -target network-extension \
    -configuration Release \
    build \
    -jobs 4 \
    CODE_SIGN_IDENTITY="" \
    CODE_SIGNING_REQUIRED=NO \
    SIGNING_ALLOWED=NO \
    OTHER_LDFLAGS="-force_load $RUST_LIB_PATH -framework Security -framework NetworkExtension -framework SystemConfiguration -framework CoreFoundation" \
    -arch "$XCODE_ARCH" \
    SYMROOT="$NE_PROJECT_DIR/build" \
    > build_appex_quick.log 2>&1 || echo "⚠️ Network Extension Appex rebuild failed (check log)"

APPEX_PATH=$(find "$NE_PROJECT_DIR/build" -name "*.appex" | head -n 1)
if [ -n "$APPEX_PATH" ]; then
    echo "✅ NE Appex found at $APPEX_PATH"
else
    echo "⚠️ NE Appex not found."
fi

echo "✅ Update Complete. Please restart your client/dev server to pick up changes."
