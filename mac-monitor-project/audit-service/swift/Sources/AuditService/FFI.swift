import Foundation

@_silgen_name("init_audit_core")
func rust_init_audit_core()

@_silgen_name("analyze_enhanced_image")
func rust_analyze_enhanced_image(
    _ ptr: UnsafePointer<UInt8>,
    _ len: Int,
    _ width: UInt32,
    _ height: UInt32,
    _ app_name: UnsafePointer<CChar>,
    _ is_sensitive: Bool,
    _ ocr_text: UnsafePointer<CChar>,
    _ redaction_labels: UnsafePointer<CChar>
)

@_silgen_name("log_audit_event")
func rust_log_audit_event(_ event_json: UnsafePointer<CChar>)

@_silgen_name("export_root_ca_pem")
func rust_export_root_ca_pem(_ buffer: UnsafeMutablePointer<UInt8>, _ buffer_len: Int) -> Int32

@_silgen_name("register_device")
func rust_register_device(
    _ server_ip: UnsafePointer<CChar>,
    _ server_port: UnsafePointer<CChar>,
    _ cpe_id: UnsafePointer<CChar>,
    _ pin: UnsafePointer<CChar>
) -> Bool

@_silgen_name("log_clipboard_event")
func rust_log_clipboard_event(
    _ app_name: UnsafePointer<CChar>,
    _ bundle_id: UnsafePointer<CChar>,
    _ content: UnsafePointer<CChar>,
    _ content_type: UnsafePointer<CChar>,
    _ risk_level: Int32
)

@_silgen_name("update_redaction_status")
func rust_update_redaction_status(_ enabled: Bool) {
    if #available(macOS 12.3, *) {
        ScreenCapturer.shared.redactionEnabled = enabled
        print("🛡 Redaction status updated via IPC: \(enabled)")
    }
}
