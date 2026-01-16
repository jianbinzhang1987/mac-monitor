import Foundation

print("🚀 Audit Service Launcher Initializing...")

// FFI functions are defined in FFI.swift

// Start non-ES services only (without EndpointSecurity)
func startAuditServices() {
    print("🛡 Starting Audit Services (non-ES mode)...")

    // Initialize Rust Core (Starts Scanner and Sync Service)
    rust_init_audit_core()

    // Start IPC Server
    // AuditIPCServer.shared.start()
    
    // Start Screen Capturer (if macOS 12.3+)
    if #available(macOS 12.3, *) {
        ScreenCapturer.shared.start()
    } else {
        print("⚠️ Screen Capture requires macOS 12.3+")
    }
    
    print("✅ Audit Services started")
    print("📍 Socket location: /tmp/mac_monitor_audit.sock")
}

startAuditServices()

// Prevent the process from exiting
CFRunLoopRun()
