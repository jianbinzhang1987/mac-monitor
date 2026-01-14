import Foundation
import EndpointSecurity

class AuditMonitor {
    private var client: OpaquePointer?

    // 定义受保护的路径 (生产环境应动态获取 Bundle 路径)
    private let protectedPaths = [
        "/Library/Application Support/MacMonitor",
        "/var/log/macmonitor",
        Bundle.main.bundlePath
    ]

    // 定义允许操作的受信任进程名 (白名单)
    private let trustedProcessNames = [
        "AuditService",
        "NetworkExtension",
        "MacMonitorApp",
        "com.apple.driver.EndpointSecurity.SystemExtension" // 系统进程
    ]

    init() {
        // 创建 ES Client
        let result = es_new_client(&client) { [weak self] (client, message) in
            self?.handleMessage(client, message)
        }

        if result != ES_NEW_CLIENT_RESULT_SUCCESS {
            print("Failed to create ES client: \(result)")
            return
        }

        subscribeToEvents()
        print("Endpoint Security Monitoring & Protection Started")
    }

    private func subscribeToEvents() {
        guard let client = client else { return }

        // 混合订阅：AUTH用于阻断保护，NOTIFY用于审计记录
        let events = [
            // --- 保护类 (Auth) ---
            ES_EVENT_TYPE_AUTH_OPEN,       // 防止读取/修改敏感文件
            ES_EVENT_TYPE_AUTH_UNLINK,     // 防止删除文件
            ES_EVENT_TYPE_AUTH_RENAME,     // 防止重命名
            ES_EVENT_TYPE_AUTH_SIGNAL,     // 防止被杀进程 (Kill)

            // --- 审计类 (Notify) ---
            ES_EVENT_TYPE_NOTIFY_EXEC,     // 进程启动
            ES_EVENT_TYPE_NOTIFY_EXIT,     // 进程退出
            ES_EVENT_TYPE_NOTIFY_WRITE,    // 文件写入
            ES_EVENT_TYPE_NOTIFY_MOUNT     // 挂载设备
        ]

        let esRet = es_subscribe(client, events, UInt32(events.count))
        if esRet != ES_RETURN_SUCCESS {
            print("Failed to subscribe to ES events: \(esRet)")
        }
    }

    private func handleMessage(_ client: OpaquePointer, _ message: UnsafePointer<es_message_t>) {
        let eventType = message.pointee.event_type

        // 1. 处理需要授权的事件 (Protection Logic)
        if isAuthEvent(eventType) {
            handleAuthEvent(client, message)
            return
        }

        // 2. 处理通知事件 (Audit Logic)
        switch eventType {
        case ES_EVENT_TYPE_NOTIFY_EXEC:
            let proc = message.pointee.event.exec
            let processName = getProcessName(proc.target)
            let timestamp = getCurrentTimestamp()

            // 构造 JSON 日志
            let json = """
            {
                "type": "exec",
                "proc": "\(processName)",
                "op_time": "\(timestamp)",
                "host_id": "host_123",
                "op_type": "exec"
            }
            """
            logEvent(json)

        case ES_EVENT_TYPE_NOTIFY_WRITE:
            let write = message.pointee.event.write
            let path = getPath(write.target)
            let processName = getProcessName(message.pointee.process)
            let timestamp = getCurrentTimestamp()

            let json = """
            {
                "type": "write",
                "proc": "\(processName)",
                "op_file": "\(path)",
                "op_time": "\(timestamp)",
                "op_type": "1"
            }
            """
            logEvent(json)

        default:
            break
        }
    }

    // MARK: - Protection Logic

    private func isAuthEvent(_ type: es_event_type_t) -> Bool {
        return type == ES_EVENT_TYPE_AUTH_OPEN ||
               type == ES_EVENT_TYPE_AUTH_UNLINK ||
               type == ES_EVENT_TYPE_AUTH_RENAME ||
               type == ES_EVENT_TYPE_AUTH_SIGNAL
    }

    private func handleAuthEvent(_ client: OpaquePointer, _ message: UnsafePointer<es_message_t>) {
        var shouldAllow = true
        let actingProcessName = getProcessName(message.pointee.process)

        // 如果是受信任的自身进程，直接放行
        if trustedProcessNames.contains(actingProcessName) {
            es_respond_auth_result(client, message, ES_AUTH_RESULT_ALLOW, false)
            return
        }

        let eventType = message.pointee.event_type

        // A. 文件保护逻辑
        if let targetPath = getTargetFilePath(message) {
            // 检查目标文件是否在受保护路径下
            for protectedPath in protectedPaths {
                if targetPath.hasPrefix(protectedPath) {
                    shouldAllow = false
                    print("SECURITY ALERT: Process '\(actingProcessName)' tried to modify protected file: \(targetPath). BLOCKED.")

                    // 记录攻击日志
                    let json = """
                    {
                        "type": "alert",
                        "proc": "\(actingProcessName)",
                        "op_file": "\(targetPath)",
                        "op_type": "tamper_attempt",
                        "op_ret": "blocked"
                    }
                    """
                    logEvent(json)
                    break
                }
            }
        }

        // B. 进程防杀逻辑 (Anti-Kill)
        if eventType == ES_EVENT_TYPE_AUTH_SIGNAL {
            let signal = message.pointee.event.signal
            // 获取目标进程信息
            let targetProcName = getProcessName(signal.target)

            // 如果目标是我们自己的进程，且发起者不是我们自己 -> 拦截
            if trustedProcessNames.contains(targetProcName) {
                // 允许 SIGKILL (9) 以外的信号？或者全部拦截？
                // 这里采用严格模式：外部进程禁止发送任何信号
                shouldAllow = false
                print("SECURITY ALERT: Process '\(actingProcessName)' tried to signal/kill protected process: \(targetProcName). BLOCKED.")
            }
        }

        let result = shouldAllow ? ES_AUTH_RESULT_ALLOW : ES_AUTH_RESULT_DENY
        es_respond_auth_result(client, message, result, false)
    }

    // MARK: - Helpers

    private func logEvent(_ json: String) {
        // 去除换行符使 JSON 紧凑
        let compactJson = json.replacingOccurrences(of: "\n", with: "").replacingOccurrences(of: "  ", with: "")
        compactJson.withCString { ptr in
            log_audit_event(ptr)
        }
    }

    private func getProcessName(_ process: UnsafePointer<es_process_t>) -> String {
        let path = getPath(process.pointee.executable)
        return (path as NSString).lastPathComponent
    }

    private func getPath(_ file: UnsafePointer<es_file_t>) -> String {
        let pathData = file.pointee.path
        return String(cString: pathData.data)
    }

    private func getTargetFilePath(_ message: UnsafePointer<es_message_t>) -> String? {
        switch message.pointee.event_type {
        case ES_EVENT_TYPE_AUTH_OPEN:
            return getPath(message.pointee.event.open.file)
        case ES_EVENT_TYPE_AUTH_UNLINK:
            return getPath(message.pointee.event.unlink.target)
        case ES_EVENT_TYPE_AUTH_RENAME:
             return getPath(message.pointee.event.rename.source)
        default:
            return nil
        }
    }

    private func getCurrentTimestamp() -> String {
        let formatter = ISO8601DateFormatter()
        return formatter.string(from: Date())
    }
}

// FFI Declarations for Rust core
@_silgen_name("log_audit_event")
func log_audit_event(_ event_json: UnsafePointer<Int8>)

