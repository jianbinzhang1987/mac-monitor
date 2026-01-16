import NetworkExtension
import Foundation

@objc class VPNManager: NSObject {
    @objc static let shared = VPNManager()

    private var manager: NETunnelProviderManager?
    private let deviceInfoFilename = "device_info.json"
    private let auditPolicyFilename = "audit_policy.json"

    override init() {
        super.init()
    }

    func loadManager(completion: @escaping (NETunnelProviderManager?) -> Void) {
        print("[Debug] Loading all managers...")
        NETunnelProviderManager.loadAllFromPreferences { [weak self] (managers, error) in
            if let error = error {
                print("[Debug] Error loading managers: \(error)")
                completion(nil)
                return
            }
            let manager = managers?.first ?? NETunnelProviderManager()
            self?.manager = manager
            print("[Debug] Manager loaded. Enabled: \(manager.isEnabled)")
            completion(manager)
        }
    }

    @objc func startVPN() {
        loadManager { [weak self] manager in
            guard let self = self, let manager = manager else { return }
            
            // Explicitly reload to avoid stale config
            print("[Debug] Reloading manager preferences...")
            manager.loadFromPreferences { error in
                if let error = error {
                    print("[Debug] Load error: \(error)")
                    // Continue anyway, maybe it's a new manager
                }
                
                self.configureAndSave(manager: manager)
            }
        }
    }

    private func configureAndSave(manager: NETunnelProviderManager) {
        let protocolConfiguration = (manager.protocolConfiguration as? NETunnelProviderProtocol) ?? NETunnelProviderProtocol()
        
        protocolConfiguration.providerBundleIdentifier = "com.mac-monitor-gui.app.network-extension"
        protocolConfiguration.serverAddress = "MacMonitorAuditSystem"
        
        var providerConfig = [String: NSObject]()
        if let info = loadDeviceInfo() {
            for (key, value) in deviceInfoDictionary(from: info) {
                providerConfig[key] = value as NSString
            }
        }
        if let policy = loadAuditPolicy() {
            providerConfig["audit_policy_json"] = policy as NSString
        }
        
        if !providerConfig.isEmpty {
            protocolConfiguration.providerConfiguration = providerConfig
        }

        manager.protocolConfiguration = protocolConfiguration
        manager.localizedDescription = "Mac Monitor VPN"
        manager.isEnabled = true

        print("[Debug] Saving preferences...")
        manager.saveToPreferences { error in
            if let error = error {
                print("[Debug] Failed to save VPN preferences: \(error)")
                // If stale, we might want to retry? For now just log.
            } else {
                print("[Debug] VPN preferences saved successfully")
            }
            
            // Reload again before starting
            manager.loadFromPreferences { error in
                if let error = error {
                     print("[Debug] Post-save reload error: \(error)")
                }
                self.performStart(manager: manager)
            }
        }
    }
    
    private func performStart(manager: NETunnelProviderManager) {
        print("[Debug] Attempting to start VPN tunnel...")
        var options = [String: NSObject]()
        if let info = self.loadDeviceInfo() {
            for (key, value) in self.deviceInfoDictionary(from: info) {
                options[key] = value as NSString
            }
        }
        if let policy = self.loadAuditPolicy() {
            options["audit_policy_json"] = policy as NSString
        }
        do {
            try manager.connection.startVPNTunnel(options: options)
            print("[Debug] startVPNTunnel called successfully.")
        } catch {
            print("[Debug] Failed to start VPN: \(error)")
        }
    }

    @objc func stopVPN() {
        loadManager { manager in
            manager?.connection.stopVPNTunnel()
            print("VPN stopping...")
        }
    }
    
    @objc func printStatus() {
         loadManager { manager in
             print("VPN Status: \(manager?.connection.status.rawValue ?? 0)")
         }
    }

    private func loadDeviceInfo() -> DeviceInfo? {
        guard let baseDir = applicationSupportDirectory() else { return nil }
        let fileURL = baseDir.appendingPathComponent(deviceInfoFilename)
        guard let data = try? Data(contentsOf: fileURL) else { return nil }
        return try? JSONDecoder().decode(DeviceInfo.self, from: data)
    }

    private func loadAuditPolicy() -> String? {
        guard let baseDir = applicationSupportDirectory() else { return nil }
        let fileURL = baseDir.appendingPathComponent(auditPolicyFilename)
        guard let data = try? Data(contentsOf: fileURL) else { return nil }
        return String(data: data, encoding: .utf8)
    }

    private func applicationSupportDirectory() -> URL? {
        guard let baseDir = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first else {
            return nil
        }
        let bundleId = Bundle.main.bundleIdentifier ?? "mac-monitor-gui"
        return baseDir.appendingPathComponent(bundleId, isDirectory: true)
    }

    private func deviceInfoDictionary(from info: DeviceInfo) -> [String: String] {
        return [
            "pin_number": info.pin_number,
            "ip": info.ip,
            "mac": info.mac,
            "cpe_id": info.cpe_id,
            "host_id": info.host_id
        ]
    }
}

private struct DeviceInfo: Codable {
    let pin_number: String
    let ip: String
    let mac: String
    let cpe_id: String
    let host_id: String
}
