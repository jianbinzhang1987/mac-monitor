import Foundation
import Network

/// 审计 IPC 服务端
/// 负责监听 /tmp/mac_monitor_audit.sock 并接收来自 Network Extension 和 GUI 的数据
class AuditIPCServer {
    static let shared = AuditIPCServer()
    
    private let socketPath = "/tmp/mac_monitor_audit.sock"
    private var listener: NWListener?
    
    func start() {
        print("🔌 Audit IPC Server: Starting on \(socketPath)...")
        
        // 清理旧的套接字文件
        unlink(socketPath)
        
        do {
            // 使用 Network.framework 监听 Unix Domain Socket
            // 注意：在 macOS 12+ 建议使用 NWListener
            let endpoint = NWEndpoint.unix(path: socketPath)
            let params = NWParameters.tcp
            // 这里通常需要自定义参数来支持 Unix Domain Socket，
            // 简化演示：我们使用更传统的 POSIX Socket 接口实现
            
            startPosixSocketServer()
            
        } catch {
            print("❌ Failed to start IPC Server: \(error)")
        }
    }
    
    private func startPosixSocketServer() {
        DispatchQueue.global(qos: .background).async {
            let serverFd = socket(AF_UNIX, SOCK_STREAM, 0)
            guard serverFd >= 0 else { return }
            
            var addr = sockaddr_un()
            addr.sun_family = sa_family_t(AF_UNIX)
            let pathLen = self.socketPath.utf8.count
            _ = withUnsafeMutablePointer(to: &addr.sun_path) { ptr in
                self.socketPath.withCString { cstr in
                    memcpy(ptr, cstr, pathLen)
                }
            }
            
            let len = socklen_t(MemoryLayout<sa_family_t>.size + pathLen + 1)
            
            unlink(self.socketPath)
            
            guard bind(serverFd, UnsafeRawPointer(&addr).assumingMemoryBound(to: sockaddr.self), len) >= 0 else {
                print("❌ Bind failed")
                return
            }
            
            // 设置权限，允许 GUI 和 NE 访问
            chmod(self.socketPath, 0o666)
            
            listen(serverFd, 5)
            
            while true {
                let clientFd = accept(serverFd, nil, nil)
                if clientFd >= 0 {
                    self.handleClient(clientFd)
                }
            }
        }
    }
    
    private func handleClient(_ fd: Int32) {
        print("🤝 New IPC connection accepted (fd: \(fd))")
        DispatchQueue.global(qos: .utility).async {
            var buffer = [UInt8](repeating: 0, count: 65536)
            let bytesRead = read(fd, &buffer, buffer.count)

            var response = """
            {"status":"error","message":"Invalid request"}
            """

            if bytesRead > 0 {
                let data = Data(buffer.prefix(bytesRead))
                let requestStr = String(data: data, encoding: .utf8) ?? "binary data"
                print("📥 Received raw IPC data (\(bytesRead) bytes): \(requestStr)")

                if let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any] {
                    response = self.processCommand(json, clientFd: fd)
                } else {
                    print("⚠️ Failed to parse JSON from IPC data")
                }
            } else {
                print("⚠️ Read 0 bytes from IPC connection")
            }

            // 发送响应
            print("📤 Sending IPC response: \(response)")
            response.withCString { ptr in
                let len = strlen(ptr)
                write(fd, ptr, len)
            }

            close(fd)
            print("👋 IPC connection closed (fd: \(fd))")
        }
    }
    
    private func processCommand(_ json: [String: Any], clientFd: Int32) -> String {
        guard let command = json["command"] as? String else {
            return """
            {"status":"error","message":"Missing command"}
            """
        }
        
        let payload = json["payload"]
        
        print("📥 IPC Command received: \(command)")
        
        switch command {
        case "register":
            // 处理注册请求
            print("✅ Processing register command")
            if let payloadDict = payload as? [String: Any],
               let serverIp = payloadDict["server_ip"] as? String,
               let serverPort = payloadDict["server_port"] as? String,
               let cpeId = payloadDict["cpe_id"] as? String,
               let pin = payloadDict["pin"] as? String {

                let success = rust_register_device(serverIp, serverPort, cpeId, pin)

                if success {
                    return """
                    {"status":"ok","message":"Registration successful","payload":null}
                    """
                } else {
                    return """
                    {"status":"error","message":"Registration failed in core"}
                    """
                }
            } else {
                return """
                {"status":"error","message":"Invalid payload for register"}
                """
            }
            
        case "login":
            // 处理登录请求
            print("✅ Processing login command")
            return """
            {"status":"ok","message":"Login processed","payload":{"token":"mock_token_12345"}}
            """
            
        case "get_pops":
            // 返回 POP 节点列表
            print("✅ Processing get_pops command")
            return """
            {"status":"ok","message":"POP nodes retrieved","payload":[{"pop_id":"hk-01","name":"香港 CN2 01","latency_hint":25},{"pop_id":"sg-01","name":"新加坡 BGP 01","latency_hint":45}]}
            """
            
        case "check_update":
            // 检查更新
            print("✅ Processing check_update command")
            return """
            {"status":"ok","message":"No updates available","payload":{"has_update":false}}
            """
            
        case "log_traffic":
            // 接收来自 Network Extension 的流量审计日志
            uploadToServer(endpoint: "/log/upload", payload: payload ?? "")
            return """
            {"status":"ok","message":"Traffic log received"}
            """
            
        case "log_event":
            // 接收来自其它组件（如 FirefoxMonitor）的行为日志
            if let eventStr = payload as? String {
                uploadToServer(endpoint: "/log/upload", payload: eventStr)
            }
            return """
            {"status":"ok","message":"Event log received"}
            """
            
        default:
            print("⚠️ Unknown IPC command: \(command)")
            return """
            {"status":"error","message":"Unknown command: \(command)"}
            """
        }
    }
    
    private func uploadToServer(endpoint: String, payload: Any) {
        // TODO: 结合 AppState 获取 Server URL 和 Token
        // 这里简化为直接通过 URLSession 批量/即时上报
        print("🚀 [Audit Upload] Sending data to \(endpoint)...")
        
        // 实际上此处应调动 Rust Core 的异步上报逻辑或 Swift 侧的重试队列
    }
}
