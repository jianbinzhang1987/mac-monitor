import Foundation
import ScreenCaptureKit
import Vision
import CoreGraphics
import CoreImage
import CoreVideo
import VideoToolbox
import AppKit

// 定义协议以安全访问新版 macOS 属性，解决编译 SDK 过低和 KVC 不兼容导致的崩溃问题
@objc protocol SCStreamConfigurationPrivate {
    @objc optional func setShowsRecordingIndicator(_ show: Bool)
    @objc optional func setExcludesCurrentProcess(_ exclude: Bool)
}

// 配置结构
struct CaptureConfig: Codable {
    struct CaptureSettings: Codable {
        let fullscreen_enabled: Bool
        let window_enabled: Bool
        let capture_interval: TimeInterval
        let window_scan_interval: TimeInterval
    }

    struct FilterSettings: Codable {
        let min_window_width: CGFloat
        let min_window_height: CGFloat
        let one_window_per_app: Bool
    }

    struct OCRSettings: Codable {
        let enabled: Bool
        let language_correction: Bool
        let recognition_level: String
        let redaction_enabled: Bool
        let sensitive_keywords: [String]
    }

    struct TargetApp: Codable {
        let name: String
        let bundle_id: String
        let enabled: Bool
    }

    let capture: CaptureSettings
    let filter: FilterSettings
    let ocr: OCRSettings
    let target_apps: [TargetApp]
}

@available(macOS 12.3, *)
class ScreenCapturer: NSObject, SCStreamOutput, SCStreamDelegate {
    static let shared = ScreenCapturer()

    // 全屏截图 stream
    private var displayStream: SCStream?

    // 上次处理的时间记录
    private var lastCaptureTime: Date = Date.distantPast
    private var lastWindowProcessTime: [String: Date] = [:]

    private let videoSampleBufferQueue = DispatchQueue(label: "com.macmonitor.VideoSampleBufferQueue")


    // 定时器用于动态窗口监控
    private var windowScanTimer: Timer?

    // 配置参数 (从配置文件加载)
    private var config: CaptureConfig?
    private var captureInterval: TimeInterval = 10.0
    private var windowScanInterval: TimeInterval = 30.0
    private var minWindowWidth: CGFloat = 400
    private var minWindowHeight: CGFloat = 300
    private var targetApps: [String] = []
    var redactionEnabled: Bool = true // 改为 internal 方便修改
    private var sensitiveKeywords: [String] = []

    // 窗口信息 (仅用于坐标裁剪，不再持有流)
    struct WindowInfo {
        let appName: String
        let bundleId: String
        let windowID: CGWindowID
        let frame: CGRect
    }

    // 活跃的目标窗口列表
    private var activeWindows: [String: WindowInfo] = [:]

    // 屏幕高度 (用于坐标转换)
    private var screenHeight: Int = 1080


    // OCR 请求
    private lazy var ocrRequest: VNRecognizeTextRequest = {
        let request = VNRecognizeTextRequest()
        request.recognitionLevel = .accurate
        request.usesLanguageCorrection = true

        // 支持中文和英文混合识别
        // macOS 13.0+ 支持简体中文(zh-Hans)和繁体中文(zh-Hant)
        if #available(macOS 13.0, *) {
            request.recognitionLanguages = ["zh-Hans", "zh-Hant", "en-US"]
        } else {
            // macOS 12.x 只支持英文
            request.recognitionLanguages = ["en-US"]
        }

        // 自动检测语言(如果系统支持)
        request.automaticallyDetectsLanguage = true

        return request
    }()

    override init() {
        super.init()
        loadConfig()
    }

    private func loadConfig() {
        let configPath = "/Users/adolf/Desktop/code/clash/mac-monitor-project/audit-service/config.json"

        guard let data = try? Data(contentsOf: URL(fileURLWithPath: configPath)) else {
            print("⚠️ Config file not found, using defaults")
            return
        }

        do {
            let decoder = JSONDecoder()
            config = try decoder.decode(CaptureConfig.self, from: data)

            // 应用配置
            if let cfg = config {
                captureInterval = cfg.capture.capture_interval
                windowScanInterval = cfg.capture.window_scan_interval
                minWindowWidth = cfg.filter.min_window_width
                minWindowHeight = cfg.filter.min_window_height

                // 加载脱敏配置
                redactionEnabled = cfg.ocr.redaction_enabled
                sensitiveKeywords = cfg.ocr.sensitive_keywords

                // 提取启用的目标应用
                targetApps = cfg.target_apps
                    .filter { $0.enabled }
                    .flatMap { [$0.name, $0.bundle_id] }

                print("✅ Config loaded: \(targetApps.count/2) target apps enabled")
            }
        } catch {
            print("❌ Failed to parse config: \(error)")
        }
    }

    func start() {
        print("📸 ScreenCapturer: Starting...")

        // 1. 检查权限
        if !CGPreflightScreenCaptureAccess() {
             print("⚠️ Screen recording permission not granted! Attempting to request...")
             if !CGRequestScreenCaptureAccess() {
                 print("❌ Permission denied.")
                 return
             }
        }

        // 2. 获取可分享的内容
        Task {
            do {
                let content = try await SCShareableContent.current
                guard let display = content.displays.first else {
                    print("❌ No display found")
                    return
                }

                // 启动全屏截图
                self.startDisplayStream(display: display)

                // 启动特定应用窗口截图
                self.startWindowStreams(content: content)

                // 启动定时扫描新窗口
                self.startWindowScanTimer()

            } catch {
                print("❌ Failed to get shareable content: \(error)")
            }
        }
    }

    private func startWindowScanTimer() {
        windowScanTimer = Timer.scheduledTimer(withTimeInterval: windowScanInterval, repeats: true) { [weak self] _ in
            self?.scanForNewWindows()
        }
        print("⏰ Window scan timer started (interval: \(windowScanInterval)s)")
    }

    private func scanForNewWindows() {
        Task {
            do {
                let content = try await SCShareableContent.current
                print("🔍 Scanning for new windows...")
                await self.updateWindowStreams(content: content)
            } catch {
                print("❌ Failed to scan windows: \(error)")
            }
        }
    }

    private func updateWindowStreams(content: SCShareableContent) async {
        var currentWindowIDs = Set<String>()

        // 更新屏幕高度，供参考
        if let display = content.displays.first {
            self.screenHeight = display.height
        }

        // 1. 扫描并更新活跃窗口信息
        for window in content.windows {
            guard let app = window.owningApplication else { continue }

            let appName = app.applicationName
            let bundleId = app.bundleIdentifier

            // 检查是否是目标应用
            let isTargetApp = targetApps.contains { target in
                return appName.lowercased().contains(target.lowercased()) ||
                       bundleId.lowercased().contains(target.lowercased())
            }

            guard isTargetApp else { continue }

            // 窗口尺寸过滤
            if window.frame.width < minWindowWidth || window.frame.height < minWindowHeight {
                continue
            }

            // 过滤无效窗口 (例如最小化或隐藏的窗口往往有奇怪的坐标)
            if window.frame.origin.x.isNaN || window.frame.origin.y.isNaN { continue }

            // 每应用单窗口策略: 检查该应用是否已记录了窗口
            let hasAppWindow = currentWindowIDs.contains { key in
                return key.starts(with: bundleId)
            }
            if hasAppWindow {
                continue
            }

            let streamKey = "\(bundleId)_\(window.windowID)"
            currentWindowIDs.insert(streamKey)

            // 更新窗口信息 (主要是 Frame 变化)
            let info = WindowInfo(
                appName: appName,
                bundleId: bundleId,
                windowID: window.windowID,
                frame: window.frame
            )

            if activeWindows[streamKey] == nil {
                print("  ✨ Target window detected: \(appName) (\(bundleId)) Frame: \(window.frame)")
            }
            activeWindows[streamKey] = info
        }

        // 2. 清理已关闭窗口
        let windowsToRemove = activeWindows.keys.filter { !currentWindowIDs.contains($0) }
        for key in windowsToRemove {
            if let info = activeWindows[key] {
                print("  🗑 Target window closed: \(info.appName)")
            }
            activeWindows.removeValue(forKey: key)
            lastWindowProcessTime.removeValue(forKey: key)
        }
    }

    // 移除 createWindowStream，改为全屏裁剪方案，不再单独创建窗口流

    private func startDisplayStream(display: SCDisplay) {
        print("🖥 Starting full screen capture stream...")
        let filter = SCContentFilter(display: display, excludingWindows: [])

        let config = SCStreamConfiguration()
        config.width = display.width
        config.height = display.height
        config.pixelFormat = kCVPixelFormatType_32BGRA
        config.minimumFrameInterval = CMTime(value: 1, timescale: 1)
        config.queueDepth = 5

        // 进一步减少感知：隐藏光标，排除自身进程
        config.showsCursor = false

        // 使用协议映射安全设置属性，避免编译错误和运行时崩溃
        if let privateConfig = config as AnyObject as? SCStreamConfigurationPrivate {
            privateConfig.setExcludesCurrentProcess?(true)
            privateConfig.setShowsRecordingIndicator?(false)
        }

        do {
            displayStream = SCStream(filter: filter, configuration: config, delegate: self)
            try displayStream?.addStreamOutput(self, type: .screen, sampleHandlerQueue: videoSampleBufferQueue)

            displayStream?.startCapture { error in
                if let error = error {
                    print("❌ Failed to start display capture: \(error)")
                } else {
                    print("✅ Display capture stream started")
                }
            }
        } catch {
            print("❌ Error creating display stream: \(error)")
        }
    }

    private func startWindowStreams(content: SCShareableContent) {
        print("🪟 Starting window capture streams for target apps...")

        // 使用新的统一方法
        Task {
            await updateWindowStreams(content: content)
        }
    }

    func stop() {
        // 停止定时器
        windowScanTimer?.invalidate()
        windowScanTimer = nil

        // 停止全屏截图
        displayStream?.stopCapture()
        displayStream = nil

        // 清理状态
        activeWindows.removeAll()
        lastWindowProcessTime.removeAll()

        print("🛑 ScreenCapturer stopped")
    }

    // MARK: - SCStreamOutput

    func stream(_ stream: SCStream, didOutputSampleBuffer sampleBuffer: CMSampleBuffer, of type: SCStreamOutputType) {
        guard type == .screen else { return }

        // 我们现在只处理 displayStream
        guard stream === displayStream else { return }

        let now = Date()

        // 1. 全屏截图频率控制
        if now.timeIntervalSince(lastCaptureTime) >= captureInterval {
            lastCaptureTime = now
            processFrame(sampleBuffer: sampleBuffer, captureType: "fullscreen", windowInfo: nil)
        }

        // 2. 虚拟窗口截图 (从全屏流裁剪)
        processVirtualWindows(sampleBuffer: sampleBuffer)
    }

    private func processVirtualWindows(sampleBuffer: CMSampleBuffer) {
        guard !activeWindows.isEmpty else { return }
        guard let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else { return }

        // 锁定基地址以读取全屏尺寸
        CVPixelBufferLockBaseAddress(pixelBuffer, .readOnly)
        let fullWidth = CGFloat(CVPixelBufferGetWidth(pixelBuffer))
        let fullHeight = CGFloat(CVPixelBufferGetHeight(pixelBuffer))
        CVPixelBufferUnlockBaseAddress(pixelBuffer, .readOnly)

        let now = Date()

        for (key, info) in activeWindows {
            // 频率控制
            let lastTime = lastWindowProcessTime[key] ?? Date.distantPast
            if now.timeIntervalSince(lastTime) < captureInterval { continue }

            // 精准计算缩放比例 (Retina 处理)
            // 使用系统主屏幕的缩放系数，这是最可靠的方法
            let scale = NSScreen.main?.backingScaleFactor ?? (fullWidth > 2000 ? 2.0 : 1.0)

            // 裁剪后的窗口 (像素坐标)
            let x = info.frame.origin.x * scale
            let y = info.frame.origin.y * scale
            let w = info.frame.width * scale
            let h = info.frame.height * scale
            let pixelFrame = CGRect(x: x, y: y, width: w, height: h)

            let intersectRect = pixelFrame.intersection(CGRect(x: 0, y: 0, width: fullWidth, height: fullHeight))

            if intersectRect.width < 50 || intersectRect.height < 50 { continue }

            lastWindowProcessTime[key] = now

            // 处理裁剪后的窗口
            processFrame(sampleBuffer: sampleBuffer, captureType: "window[\(info.appName)]", windowInfo: info, scale: scale)
        }
    }

    private func processFrame(sampleBuffer: CMSampleBuffer, captureType: String, windowInfo: WindowInfo?, scale: CGFloat = 1.0) {
        guard let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else { return }

        CVPixelBufferLockBaseAddress(pixelBuffer, .readOnly)
        defer { CVPixelBufferUnlockBaseAddress(pixelBuffer, .readOnly) }

        let baseAddress = CVPixelBufferGetBaseAddress(pixelBuffer)
        let fullWidth = CVPixelBufferGetWidth(pixelBuffer)
        let fullHeight = CVPixelBufferGetHeight(pixelBuffer)
        let bytesPerRow = CVPixelBufferGetBytesPerRow(pixelBuffer)

        guard let addr = baseAddress else { return }

        // 准备数据给 Rust 和 OCR
        // 如果是全屏，直接使用原始 buffer
        // 如果是窗口，我们需要拷贝出 ROI (Region of Interest)

        var targetPtr = addr
        var targetWidth = UInt32(fullWidth)
        var targetHeight = UInt32(fullHeight)
        var targetData: Data? = nil // 用于保持拷贝数据的生命周期

        if let info = windowInfo {
            // 执行裁剪拷贝 (注意：info.frame 是点坐标，必须转换为像素坐标进行物理裁剪)
            let x = Int(max(0, info.frame.origin.x * scale))
            let y = Int(max(0, info.frame.origin.y * scale))
            let w = Int(min(CGFloat(fullWidth) - CGFloat(x), info.frame.width * scale))
            let h = Int(min(CGFloat(fullHeight) - CGFloat(y), info.frame.height * scale))

            if w <= 0 || h <= 0 { return }

            targetWidth = UInt32(w)
            targetHeight = UInt32(h)

            // 创建紧凑的 buffer (bytesPerRow = w * 4)
            let newBytesPerRow = w * 4
            var newData = Data(count: h * newBytesPerRow)

            newData.withUnsafeMutableBytes { destBytes in
                guard let destBase = destBytes.baseAddress else { return }
                let srcRaw = addr.assumingMemoryBound(to: UInt8.self)

                for row in 0..<h {
                    let srcOffset = (y + row) * bytesPerRow + (x * 4)
                    let dstOffset = row * newBytesPerRow

                    // 拷贝一行
                    destBase.advanced(by: dstOffset).copyMemory(
                        from: srcRaw.advanced(by: srcOffset),
                        byteCount: w * 4
                    )
                }
            }

            targetData = newData
            targetData?.withUnsafeBytes { ptr in
                if let base = ptr.baseAddress {
                    targetPtr = UnsafeMutableRawPointer(mutating: base)
                }
            }
        }

        // 2. 执行 OCR (使用 Vision)
        // Vision 可以处理全屏图像并指定 regionOfInterest，比物理拷贝更快
        // 但为了代码复用，如果已经拷贝了 crop 数据，直接用 crop 数据做 OCR 也可以
        // 这里为了简单，我们用裁剪后的数据生成 CIImage (如果 targetData 存在)
        // 或者对全屏使用 ROI。为了逻辑统一，我们用 targetPtr 创建 CIImage。

        let ciImage: CIImage
        if let data = targetData {
             // 从裁剪后的数据创建 CIImage
             let size = CGSize(width: Int(targetWidth), height: Int(targetHeight))
             ciImage = CIImage(bitmapData: data, bytesPerRow: Int(targetWidth) * 4, size: size, format: .BGRA8, colorSpace: nil)
        } else {
             ciImage = CIImage(cvPixelBuffer: pixelBuffer)
        }

        let handler = VNImageRequestHandler(ciImage: ciImage, options: [:])

        var ocrText = ""
        var isSensitiveFrame = false
        var redactionLabels = ""
        do {
            try handler.perform([ocrRequest])
            if let observations = ocrRequest.results {
                // 1. 脱敏检测
                if redactionEnabled {
                    let targets = SensitiveInfoDetector.detect(
                        in: observations,
                        imageSize: CGSize(width: Int(targetWidth), height: Int(targetHeight)),
                        customKeywords: sensitiveKeywords
                    )

                    if !targets.isEmpty {
                        isSensitiveFrame = true
                        // 提取唯一的脱敏标签
                        redactionLabels = Array(Set(targets.map { $0.label })).joined(separator: ",")
                        print("🛡 Detected sensitive areas [\(redactionLabels)]. Redacting...")

                        // 如果是全屏且没有 targetData (即没有进行裁剪拷贝)，我们需要创建一个拷贝进行脱敏
                        if targetData == nil {
                            let size = Int(CVPixelBufferGetDataSize(pixelBuffer))
                            var copyData = Data(count: size)
                            copyData.withUnsafeMutableBytes { dest in
                                guard let destBase = dest.baseAddress else { return }
                                destBase.copyMemory(from: addr, byteCount: size)
                            }
                            targetData = copyData
                            targetData?.withUnsafeBytes { ptr in
                                if let base = ptr.baseAddress {
                                    targetPtr = UnsafeMutableRawPointer(mutating: base)
                                }
                            }
                        }

                        // 执行物理脱敏 (遮盖像素)
                        let rawPtr = UnsafeMutableRawPointer(targetPtr)
                        let mutablePtr = rawPtr.assumingMemoryBound(to: UInt8.self)

                        // 关键修复：传入正确的 bytesPerRow
                        // 如果是新分配的 targetData，则使用紧凑的 width * 4
                        // 如果是直接操作或拷贝的 pixelBuffer 数据，则使用原 buffer 的 bytesPerRow
                        let effectiveBytesPerRow = (targetData != nil && windowInfo != nil) ? (Int(targetWidth) * 4) : bytesPerRow

                        ImageRedactor.redact(
                            ptr: mutablePtr,
                            width: Int(targetWidth),
                            height: Int(targetHeight),
                            bytesPerRow: effectiveBytesPerRow,
                            targets: targets
                        )
                    }
                }

                let recognizedStrings = observations.compactMap { $0.topCandidates(1).first?.string }
                let rawOcrText = recognizedStrings.joined(separator: " ")
                ocrText = redactionEnabled ? SensitiveInfoDetector.redactText(rawOcrText, keywords: sensitiveKeywords) : rawOcrText
            }
        } catch {
            print("⚠️ OCR failed: \(error)")
        }

        // 3. 计算数据长度
        let totalBytes = (targetData != nil) ? targetData!.count : CVPixelBufferGetDataSize(pixelBuffer)

        print("📸 Capture [\(captureType)] frame: \(targetWidth)x\(targetHeight), OCR len: \(ocrText.count), sensitive: \(isSensitiveFrame) [\(redactionLabels)]")

        // 4. 获取当前最前端的应用名称
        let frontAppName = NSWorkspace.shared.frontmostApplication?.localizedName ?? "Unknown"

        // 5. 调用 FFI
        let appNameWithType = "\(frontAppName)[\(captureType)]"

        ocrText.withCString { ocrPtr in
            appNameWithType.withCString { appNamePtr in
                redactionLabels.withCString { labelsPtr in
                    rust_analyze_enhanced_image(
                        targetPtr.assumingMemoryBound(to: UInt8.self),
                        Int(totalBytes),
                        targetWidth,
                        targetHeight,
                        appNamePtr,
                        isSensitiveFrame,
                        ocrPtr,
                        labelsPtr
                    )
                }
            }
        }
    }
}
