import NetworkExtension
import Foundation

class PacketTunnelProvider: NEPacketTunnelProvider {

    // 专用队列用于处理出站数据包，避免阻塞主线程
    private let outputQueue = DispatchQueue(label: "com.macmonitor.packetTunnel.output")
    private var isRunning = false

    override func startTunnel(options: [String : NSObject]?, completionHandler: @escaping (Error?) -> Void) {
        // 1. 初始化 Rust 协议栈
        let initResult = init_stack()
        if initResult != 0 {
            completionHandler(NSError(domain: "PacketTunnel", code: 1, userInfo: [NSLocalizedDescriptionKey: "Failed to initialize Rust stack"]))
            return
        }

        // 1.1 同步设备信息到 Rust（用于审计日志字段填充）
        let pin = stringConfigValue(key: "pin_number", options: options) ?? ""
        let ip = stringConfigValue(key: "ip", options: options) ?? ""
        let mac = stringConfigValue(key: "mac", options: options) ?? ""
        let cpeId = stringConfigValue(key: "cpe_id", options: options) ?? ""
        let hostId = stringConfigValue(key: "host_id", options: options) ?? ""
        let policyJson = stringConfigValue(key: "audit_policy_json", options: options) ?? ""
        pin.withCString { pinPtr in
            ip.withCString { ipPtr in
                mac.withCString { macPtr in
                    cpeId.withCString { cpePtr in
                        hostId.withCString { hostPtr in
                            set_device_info(pinPtr, ipPtr, macPtr, cpePtr, hostPtr)
                        }
                    }
                }
            }
        }
        if !policyJson.isEmpty {
            policyJson.withCString { policyPtr in
                _ = set_audit_policy(policyPtr)
            }
        }

        let networkSettings = NEPacketTunnelNetworkSettings(tunnelRemoteAddress: "127.0.0.1")

        // Configure IPv4 settings
        // 使用 10.0.0.2 作为虚拟 IP，10.0.0.1 作为网关（在 Rust 侧配置）
        let ipv4Settings = NEIPv4Settings(addresses: ["10.0.0.2"], subnetMasks: ["255.255.255.0"])
        ipv4Settings.includedRoutes = [NEIPv4Route.default()]
        networkSettings.ipv4Settings = ipv4Settings

        // Configure DNS settings (Requirement: Clear DNS cache on login/logout)
        let dnsSettings = NEDNSSettings(servers: ["8.8.8.8", "1.1.1.1"])
        dnsSettings.matchDomains = [""] // Intercept all DNS
        networkSettings.dnsSettings = dnsSettings

        setTunnelNetworkSettings(networkSettings) { error in
            if let error = error {
                completionHandler(error)
                return
            }

            self.isRunning = true

            // 2. 开始读取入站数据包 (System -> Rust)
            self.readPackets()

            // 3. 开始轮询出站数据包 (Rust -> System)
            self.startOutputLoop()

            completionHandler(nil)
        }
    }

    override func stopTunnel(with reason: NEProviderStopReason, completionHandler: @escaping () -> Void) {
        self.isRunning = false
        shutdown_stack()
        completionHandler()
    }

    private func readPackets() {
        // 递归读取系统发送的数据包
        packetFlow.readPackets { [weak self] (packets, protocols) in
            guard let self = self, self.isRunning else { return }

            for packet in packets {
                // Call Rust FFI to process packet
                packet.withUnsafeBytes { ptr in
                    if let baseAddress = ptr.baseAddress {
                        let _ = process_packet(baseAddress.assumingMemoryBound(to: UInt8.self), ptr.count)
                    }
                }
            }

            // 驱动协议栈的一轮处理
            poll_stack()

            // Continue reading
            self.readPackets()
        }
    }

    private func startOutputLoop() {
        outputQueue.async { [weak self] in
            guard let self = self else { return }

            // 预分配缓冲区
            let bufferSize = 65536
            var buffer = [UInt8](repeating: 0, count: bufferSize)

            while self.isRunning {
                var writtenLen: Int = 0

                // 尝试从 Rust 协议栈读取数据
                let result = get_outbound_packet(&buffer, bufferSize, &writtenLen)

                if result == 0 && writtenLen > 0 {
                    let data = Data(bytes: buffer, count: writtenLen)
                    // IPv4 protocol number is 2 (NSNumber) for writePackets
                    // 但 writePackets 只接受 [Data] 和 [NSNumber]
                    self.packetFlow.writePackets([data], withProtocols: [NSNumber(value: AF_INET)])
                } else {
                    // 如果没有数据，短暂休眠避免空转占用 CPU
                    // 在高性能场景可使用条件变量或 semaphore 优化
                    poll_stack() // 确保定时器等事件被处理
                    Thread.sleep(forTimeInterval: 0.005) // 5ms
                }
            }
        }
    }

    private func stringConfigValue(key: String, options: [String: NSObject]?) -> String? {
        if let value = options?[key] as? String {
            return value
        }
        if let value = options?[key] as? NSString {
            return value as String
        }
        if let providerConfig = (protocolConfiguration as? NETunnelProviderProtocol)?.providerConfiguration {
            if let value = providerConfig[key] as? String {
                return value
            }
            if let value = providerConfig[key] as? NSString {
                return value as String
            }
        }
        return nil
    }
}

// FFI Declarations for Rust core
@_silgen_name("init_stack")
func init_stack() -> Int32

@_silgen_name("shutdown_stack")
func shutdown_stack()

@_silgen_name("process_packet")
func process_packet(_ data: UnsafePointer<UInt8>, _ len: Int) -> Int32

@_silgen_name("get_outbound_packet")
func get_outbound_packet(_ buffer: UnsafeMutablePointer<UInt8>, _ max_len: Int, _ written_len: UnsafeMutablePointer<Int>) -> Int32

@_silgen_name("poll_stack")
func poll_stack()

@_silgen_name("set_device_info")
func set_device_info(
    _ pin_number: UnsafePointer<CChar>,
    _ ip: UnsafePointer<CChar>,
    _ mac: UnsafePointer<CChar>,
    _ cpe_id: UnsafePointer<CChar>,
    _ host_id: UnsafePointer<CChar>
)

@_silgen_name("set_audit_policy")
func set_audit_policy(_ policy_json: UnsafePointer<CChar>) -> Int32
