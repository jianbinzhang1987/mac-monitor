import Foundation
import CoreGraphics
import AppKit

class ImageRedactor {
    /// 直接在原始内存缓冲区上应用遮盖
    /// - Parameters:
    ///   - ptr: 图像数据指针
    ///   - width: 图像宽度 (像素)
    ///   - height: 图像高度 (像素)
    ///   - bytesPerRow: 每行字节数 (处理内存对齐)
    ///   - targets: 需要脱敏的目标区域 (坐标系：左下角为原点，Y 轴向上，匹配 Vision)
    static func redact(ptr: UnsafeMutablePointer<UInt8>, width: Int, height: Int, bytesPerRow: Int, targets: [SensitiveInfoDetector.RedactionTarget]) {
        guard !targets.isEmpty else { return }

        let colorSpace = CGColorSpaceCreateDeviceRGB()
        // BGRA 格式 (对应 CVPixelBuffer 常见的 kCVPixelFormatType_32BGRA)
        let bitmapInfo = CGImageAlphaInfo.premultipliedFirst.rawValue | CGBitmapInfo.byteOrder32Little.rawValue

        guard let context = CGContext(
            data: ptr,
            width: width,
            height: height,
            bitsPerComponent: 8,
            bytesPerRow: bytesPerRow,
            space: colorSpace,
            bitmapInfo: bitmapInfo
        ) else {
            print("❌ Failed to create CGContext for redaction")
            return
        }

        // --- 核心修复：对齐坐标系 ---
        // SensitiveInfoDetector.convert() 已经将 Vision 的归一化坐标（左下角原点）
        // 转换为像素坐标（左上角原点），所以这里 CGContext 也需要使用左上角原点。
        // 通过 Y 轴翻转让 CGContext 的坐标系与像素内存布局一致。
        context.translateBy(x: 0, y: CGFloat(height))
        context.scaleBy(x: 1.0, y: -1.0)

        // 设置打码颜色为黑色
        context.setFillColor(NSColor.black.cgColor)

        for target in targets {
            // target.rect 已经是左上角像素坐标，可以直接使用
            let padding: CGFloat = 2.0
            let redactedRect = target.rect.insetBy(dx: -padding, dy: -padding)
            context.fill(redactedRect)
        }

        print("🛡 Successfully redacted \(targets.count) areas in image buffer")
    }
}
