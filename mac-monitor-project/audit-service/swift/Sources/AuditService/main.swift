import Foundation

print("🚀 Audit Service Launcher Initializing...")

// Mock all FFI functions
@_silgen_name("log_audit_event")
public func log_audit_event(_ event_json: UnsafePointer<Int8>) {
    let str = String(cString: event_json)
    print("📝 [MOCK FFI] log_audit_event: \(str)")
}

@_silgen_name("export_root_ca_pem")
public func export_root_ca_pem() -> UnsafePointer<Int8>? {
    print("📝 [MOCK FFI] export_root_ca_pem called")
    return nil
}

// Start non-ES services only (without EndpointSecurity)
func startAuditServices() {
    print("🛡 Starting Audit Services (non-ES mode)...")
    
    // Start IPC Server
    AuditIPCServer.shared.start()
    
    print("✅ Audit IPC Server started")
    print("📍 Socket location: /tmp/mac_monitor_audit.sock")
}

startAuditServices()

// Prevent the process from exiting
CFRunLoopRun()
