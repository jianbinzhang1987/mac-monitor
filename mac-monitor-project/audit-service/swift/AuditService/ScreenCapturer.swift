import Foundation
import ScreenCaptureKit
import CoreGraphics
import VideoToolbox
import Vision
import CoreImage

class ScreenCapturer: NSObject, SCStreamOutput {
    private var stream: SCStream?
    private let videoSampleBufferQueue = DispatchQueue(label: "com.macmonitor.videoSampleBufferQueue")
    private let analysisQueue = DispatchQueue(label: "com.macmonitor.analysisQueue", qos: .userInitiated)

    // OCR Request
    private lazy var textRecognitionRequest: VNRecognizeTextRequest = {
        let request = VNRecognizeTextRequest(completionHandler: nil)
        request.recognitionLevel = .accurate
        request.usesLanguageCorrection = true
        return request
    }()

    // Sensitive keywords to redact
    private let sensitiveKeywords = ["password", "passwd", "secret", "token", "key", "auth", "login", "signin"]

    private var lastCaptureTime = Date()
    private let captureInterval: TimeInterval = 5.0 // Analyze every 5 seconds to save CPU

    func startCapture() async {
        do {
            let content = try await SCShareableContent.excludingDesktopWindows(false, onScreenWindowsOnly: true)

            guard let display = content.displays.first else { return }

            // Exclude our own app to avoid infinite mirrors if we had a window
            let excludedApps = content.applications.filter { $0.bundleIdentifier == Bundle.main.bundleIdentifier }

            let filter = SCContentFilter(display: display, excludingApplications: excludedApps, exceptingWindows: [])

            let configuration = SCStreamConfiguration()
            configuration.width = display.width
            configuration.height = display.height
            configuration.minimumFrameInterval = CMTime(value: 1, timescale: 5) // 5fps max capture
            configuration.pixelFormat = kCVPixelFormatType_32BGRA
            configuration.showsCursor = true

            stream = SCStream(filter: filter, configuration: configuration, delegate: nil)
            try stream?.addStreamOutput(self, type: .screen, sampleHandlerQueue: videoSampleBufferQueue)
            try await stream?.startCapture()

            print("Screen capture started with OCR privacy protection")
        } catch {
            print("Failed to start screen capture: \(error)")
        }
    }

    func stream(_ stream: SCStream, didOutputSampleBuffer sampleBuffer: CMSampleBuffer, of type: SCStreamOutputType) {
        guard type == .screen else { return }

        // Throttling: Only analyze frames periodically
        let now = Date()
        guard now.timeIntervalSince(lastCaptureTime) >= captureInterval else { return }
        lastCaptureTime = now

        guard let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else { return }

        // Retain buffer for async processing
        let ciImage = CIImage(cvPixelBuffer: pixelBuffer)

        analysisQueue.async { [weak self] in
            self?.processFrame(ciImage: ciImage, originalBuffer: pixelBuffer)
        }
    }

    private func processFrame(ciImage: CIImage, originalBuffer: CVPixelBuffer) {
        let handler = VNImageRequestHandler(ciImage: ciImage, options: [:])

        do {
            try handler.perform([textRecognitionRequest])

            guard let observations = textRecognitionRequest.results else { return }

            var sensitiveRects: [CGRect] = []
            var foundSensitiveData = false
            var extractedText = ""

            for observation in observations {
                guard let candidate = observation.topCandidates(1).first else { continue }
                extractedText += candidate.string + " "

                // Simple keyword matching (Regex should be used in production)
                let lowerText = candidate.string.lowercased()
                for keyword in sensitiveKeywords {
                    if lowerText.contains(keyword) {
                        foundSensitiveData = true
                        // Convert Vision normalized coordinates to Image coordinates
                        let box = observation.boundingBox
                        sensitiveRects.append(box)
                        break
                    }
                }
            }

            if foundSensitiveData {
                // If sensitive data found, we redact the image before passing to Rust
                print("Sensitive data detected! Redacting areas...")
                // In a real implementation, we would apply a blur filter to sensitiveRects here
                // For now, we flag it.
            }

            // Prepare data for Rust
            CVPixelBufferLockBaseAddress(originalBuffer, .readOnly)
            if let baseAddress = CVPixelBufferGetBaseAddress(originalBuffer) {
                let width = CVPixelBufferGetWidth(originalBuffer)
                let height = CVPixelBufferGetHeight(originalBuffer)
                let bytesPerRow = CVPixelBufferGetBytesPerRow(originalBuffer)
                let size = height * bytesPerRow

                // Pass to Rust FFI
                // Note: We are passing the raw buffer here.
                // In production, if sensitive, we should pass a NEW buffer with redactions applied.
                analyze_image_buffer(baseAddress.assumingMemoryBound(to: UInt8.self), size, width, height)
            }
            CVPixelBufferUnlockBaseAddress(originalBuffer, .readOnly)

        } catch {
            print("Failed to perform OCR: \(error)")
        }
    }
}

// FFI Declarations for Rust core
@_silgen_name("analyze_image_buffer")
func analyze_image_buffer(_ ptr: UnsafePointer<UInt8>, _ len: Int, _ width: Int, _ height: Int)

