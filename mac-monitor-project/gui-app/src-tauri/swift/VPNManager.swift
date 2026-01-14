import NetworkExtension
import Foundation

@objc class VPNManager: NSObject {
    @objc static let shared = VPNManager()

    private var manager: NETunnelProviderManager?

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

        do {
            try manager.connection.startVPNTunnel()
            print("VPN starting...")
        } catch {
            print("Failed to start VPN: \(error)")
        }
    }

    @objc func stopVPN() {
        manager?.connection.stopVPNTunnel()
        print("VPN stopping...")
    }

    @objc func getStatus() -> Int {
        return manager?.connection.status.rawValue ?? 0
    }
}
