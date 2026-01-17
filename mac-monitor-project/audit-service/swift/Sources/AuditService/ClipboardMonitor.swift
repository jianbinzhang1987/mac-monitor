import Cocoa

class ClipboardMonitor {
    static let shared = ClipboardMonitor()

    private var timer: Timer?
    private var lastChangeCount: Int
    private let pasteboard = NSPasteboard.general

    // Browser Bundle ID Whitelist
    private let browserWhitelist: Set<String> = [
        "com.google.Chrome",
        "com.apple.Safari",
        "org.mozilla.firefox",
        "com.microsoft.edgemac",
        "company.thebrowser.Browser", // Arc
        "com.brave.Browser",
        "com.operasoftware.Opera"
    ]

    private init() {
        self.lastChangeCount = pasteboard.changeCount
    }

    func start() {
        print("📋 Clipboard Monitor: Starting...")
        // Check every 1 second
        timer = Timer.scheduledTimer(withTimeInterval: 1.0, repeats: true) { [weak self] _ in
            self?.checkClipboard()
        }
    }

    func stop() {
        timer?.invalidate()
        timer = nil
        print("📋 Clipboard Monitor: Stopped")
    }

    private func checkClipboard() {
        // 1. Check if change count incremented
        guard pasteboard.changeCount != lastChangeCount else { return }
        lastChangeCount = pasteboard.changeCount

        // 2. Identify the active application
        guard let frontApp = NSWorkspace.shared.frontmostApplication else { return }
        guard let bundleId = frontApp.bundleIdentifier else { return }

        // 3. Filter: Only log if it's a browser in the whitelist
        // Note: You can comment out this guard if you want to monitor ALL apps
        guard browserWhitelist.contains(bundleId) else {
            // Optional debug log
            // print("📋 Clipboard change ignored from non-browser: \(bundleId)")
            return
        }

        // 4. Extract content
        var content = ""
        var contentType = "unknown"

        // Priority: String -> URL -> RTF -> Other
        if let str = pasteboard.string(forType: .string) {
            content = str
            contentType = "text/plain"
        } else if let url = pasteboard.string(forType: .URL) {
            content = url
            contentType = "text/url"
        } else if pasteboard.string(forType: .rtf) != nil {
            content = "[RTF Data]" // Avoid logging raw RTF logic for now
            contentType = "application/rtf"
        } else if pasteboard.data(forType: .tiff) != nil || pasteboard.data(forType: .png) != nil {
             content = "[Image Data]"
             contentType = "image"
        }

        // Truncate if too long (e.g. 1000 chars) to avoid IPC bloat
        if content.count > 1000 {
            content = String(content.prefix(1000)) + "...(truncated)"
        }

        let appName = frontApp.localizedName ?? "Unknown"

        print("📋 Browser Copy Detected: \(appName) (\(bundleId)) - Type: \(contentType)")

        // 5. Send to Rust Core via FFI
        logToRust(appName: appName, bundleId: bundleId, content: content, contentType: contentType)
    }

    private func logToRust(appName: String, bundleId: String, content: String, contentType: String) {
        appName.withCString { cAppName in
            bundleId.withCString { cBundleId in
                content.withCString { cContent in
                    contentType.withCString { cType in
                        // Risk Level: 1 (Info/Low)
                        rust_log_clipboard_event(cAppName, cBundleId, cContent, cType, 1)
                    }
                }
            }
        }
    }
}
