import NetworkExtension
import Foundation

@objc class VPNManager: NSObject {
    @objc static let shared = VPNManager()

    private var manager: NETunnelProviderManager?
    private let deviceInfoFilename = "device_info.json"
    private let auditPolicyFilename = "audit_policy.json"

    override init() {
        super.init()
        loadManager()
    }

    func loadManager() {
        NETunnelProviderManager.loadAllFromPreferences { [weak self] (managers, error) in
            if let error = error {
                print("Failed to load VPN managers: \(error)")
                return
            }
            self?.manager = managers?.first ?? NETunnelProviderManager()
            self?.setupProtocol()
        }
    }

    private func setupProtocol() {
        guard let manager = manager else { return }

        let protocolConfiguration = NETunnelProviderProtocol()
        protocolConfiguration.providerBundleIdentifier = "com.example.mac-monitor.network-extension"
        protocolConfiguration.serverAddress = "MacMonitorAuditSystem"
        var providerConfig = [String: String]()
        if let info = loadDeviceInfo() {
            providerConfig.merge(deviceInfoDictionary(from: info)) { _, new in new }
        }
        if let policy = loadAuditPolicy() {
            providerConfig["audit_policy_json"] = policy
        }
        if !providerConfig.isEmpty {
            protocolConfiguration.providerConfiguration = providerConfig
        }

        manager.protocolConfiguration = protocolConfiguration
        manager.localizedDescription = "Mac Monitor VPN"
        manager.isEnabled = true

        manager.saveToPreferences { error in
            if let error = error {
                print("Failed to save VPN preferences: \(error)")
            } else {
                print("VPN preferences saved successfully")
            }
        }
    }

    @objc func startVPN() {
        loadManager() // Ensure we have the latest manager
        guard let manager = manager else { return }

        if let protocolConfig = manager.protocolConfiguration as? NETunnelProviderProtocol {
            var providerConfig = [String: String]()
            if let info = loadDeviceInfo() {
                providerConfig.merge(deviceInfoDictionary(from: info)) { _, new in new }
            }
            if let policy = loadAuditPolicy() {
                providerConfig["audit_policy_json"] = policy
            }
            if !providerConfig.isEmpty {
                protocolConfig.providerConfiguration = providerConfig
                manager.protocolConfiguration = protocolConfig
            }
        }

        manager.saveToPreferences { error in
            if let error = error {
                print("Failed to save VPN preferences: \(error)")
                return
            }

            var options = [String: String]()
            if let info = self.loadDeviceInfo() {
                options.merge(self.deviceInfoDictionary(from: info)) { _, new in new }
            }
            if let policy = self.loadAuditPolicy() {
                options["audit_policy_json"] = policy
            }
            do {
                try manager.connection.startVPNTunnel(options: options)
                print("VPN starting...")
            } catch {
                print("Failed to start VPN: \(error)")
            }
        }
    }

    @objc func stopVPN() {
        manager?.connection.stopVPNTunnel()
        print("VPN stopping...")
    }

    @objc func getStatus() -> Int {
        return manager?.connection.status.rawValue ?? 0
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
