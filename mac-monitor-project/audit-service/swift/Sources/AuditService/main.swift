import Foundation

print("🚀 Audit Service Launcher Initializing...")

// FFI functions are defined in FFI.swift

// Start non-ES services only (without EndpointSecurity)
func startAuditServices() {
    print("🛡 Starting Audit Services (non-ES mode)...")

    // Initialize Rust Core (Starts Scanner and Sync Service)
    // Note: Rust Core also starts the IPC server on /tmp/mac_monitor_audit.sock
    rust_init_audit_core()

    // Start Screen Capturer (if macOS 12.3+)
    if #available(macOS 12.3, *) {
        print("📸 Initializing Screen Capturer...")
        ScreenCapturer.shared.start()
    } else {
        print("⚠️ Screen Capture requires macOS 12.3+")
    }

    // Start Clipboard Monitor
    print("📋 Initializing Clipboard Monitor...")
    ClipboardMonitor.shared.start()

    print("✅ Audit Services started")
    print("📍 Socket location: /tmp/mac_monitor_audit.sock")
    print("⏳ Entering main run loop...")
}

// Set up signal handling for graceful shutdown
signal(SIGINT) { _ in
    print("\n🛑 Received SIGINT, shutting down...")
    exit(0)
}

signal(SIGTERM) { _ in
    print("\n🛑 Received SIGTERM, shutting down...")
    exit(0)
}

startAuditServices()

// Prevent the process from exiting
// Use a more robust way to keep the CLI tool alive
RunLoop.main.run()
