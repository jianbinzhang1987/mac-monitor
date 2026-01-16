import Foundation

class ProxyManager {
    static let shared = ProxyManager()
    
    // Hardcoded for Wi-Fi interface for now. Ideally should detect active interface.
    func enableSystemProxy() {
        // 1. Start traffic-proxy if not running (or ensure it's maintained by launchd/parent app)
        // For this standalone helper, we'll try to start it if we can find it nearby.
        startTrafficProxy()
        
        // 2. Trust Root Certificate
        trustRootCertificate()
        
        // 3. Set System Proxy
        runCommand("/usr/sbin/networksetup", arguments: ["-setwebproxy", "Wi-Fi", "127.0.0.1", "8050"])
        runCommand("/usr/sbin/networksetup", arguments: ["-setsecurewebproxy", "Wi-Fi", "127.0.0.1", "8050"])
        print("System Proxy Enabled (Wi-Fi -> 127.0.0.1:8050)")
    }
    
    func disableSystemProxy() {
        runCommand("/usr/sbin/networksetup", arguments: ["-setwebproxystate", "Wi-Fi", "off"])
        runCommand("/usr/sbin/networksetup", arguments: ["-setsecurewebproxystate", "Wi-Fi", "off"])
        print("System Proxy Disabled")
        stopTrafficProxy()
    }

    private func stopTrafficProxy() {
        // Kill by name using pkill, which is simplest for this helper tool
        print("Stopping traffic-proxy...")
        runCommand("/usr/bin/pkill", arguments: ["-f", "traffic-proxy"])
    }
    
    private func startTrafficProxy() {
        // Check if already running
        let pgrep = Process()
        pgrep.launchPath = "/usr/bin/pgrep"
        pgrep.arguments = ["-x", "traffic-proxy"]
        let pipe = Pipe()
        pgrep.standardOutput = pipe
        pgrep.launch()
        pgrep.waitUntilExit()
        
        let data = pipe.fileHandleForReading.readDataToEndOfFile()
        if !data.isEmpty {
            print("traffic-proxy is already running.")
            return
        }

        // Assume traffic-proxy is in the same directory as this tool
        let currentUrl = URL(fileURLWithPath: CommandLine.arguments[0])
        let binDir = currentUrl.deletingLastPathComponent()
        
        // Architecture detection is hard from Swift in this context without uname, 
        // but we know we build as vpn-helper-x86_64-apple-darwin. 
        // Let's look for known binary names.
        let candidates = [
            "traffic-proxy-x86_64-apple-darwin",
            "traffic-proxy-aarch64-apple-darwin",
            "traffic-proxy"
        ]
        
        var proxyPath: String? = nil
        for cand in candidates {
            let p = binDir.appendingPathComponent(cand).path
            if FileManager.default.fileExists(atPath: p) {
                proxyPath = p
                break
            }
        }
        
        guard let validProxyPath = proxyPath else {
            print("Warning: traffic-proxy binary not found in \(binDir.path). Proxy service might not be running.")
            return
        }
        
        // Start process
        let task = Process()
        task.launchPath = validProxyPath
        task.standardOutput = FileHandle.nullDevice
        task.standardError = FileHandle.nullDevice
        
        do {
            try task.run()
            print("Started traffic-proxy at \(validProxyPath)")
            sleep(1) // Give it a sec to bind port
        } catch {
            print("Failed to start traffic-proxy: \(error)")
        }
    }
    
    private func trustRootCertificate() {
        // Download cert from local proxy
        let certUrl = URL(string: "http://127.0.0.1:8050/ssl")!
        let tempCertPath = "/tmp/mac-monitor-ca.pem"
        
        // Synchronous download
        let semaphore = DispatchSemaphore(value: 0)
        let session = URLSession.shared
        
        let task = session.dataTask(with: certUrl) { data, response, error in
            defer { semaphore.signal() }
            if let data = data {
                do {
                    try data.write(to: URL(fileURLWithPath: tempCertPath))
                } catch {
                    print("Error saving certificate: \(error)")
                }
            } else {
                print("Error downloading certificate: \(error?.localizedDescription ?? "unknown")")
            }
        }
        task.resume()
        _ = semaphore.wait(timeout: .now() + 5)
        
        if FileManager.default.fileExists(atPath: tempCertPath) {
             print("Certificate downloaded to \(tempCertPath), adding to System Keychain (requires sudo)...")
             runCommand("/usr/bin/security", arguments: [
                 "add-trusted-cert",
                 "-d",
                 "-r", "trustRoot",
                 "-k", "/Library/Keychains/System.keychain",
                 tempCertPath
             ])
             print("Certificate trust command executed.")
        } else {
            print("Skipping certificate trust: Certificate file not found.")
        }
    }
    
    private func runCommand(_ command: String, arguments: [String]) {
        let task = Process()
        task.launchPath = command
        task.arguments = arguments
        let pipe = Pipe()
        task.standardOutput = pipe
        task.standardError = pipe
        task.launch()
        task.waitUntilExit()
        
        let data = pipe.fileHandleForReading.readDataToEndOfFile()
        if let output = String(data: data, encoding: .utf8), !output.isEmpty {
            // Filter some noisy output if needed, but for CLI tool explicit is better
            print(output)
        }
    }
}
