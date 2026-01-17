import Foundation
import Vision
import NaturalLanguage

class SensitiveInfoDetector {
    struct RedactionTarget {
        let rect: CGRect
        let label: String
    }

    // 正则表达式定义
    private static let idCardPattern = #"\d{15}(\d{2}[0-9xX])?"#
    private static let phonePattern = #"(?:\+86)?1[3-9]\d{9}"#
    private static let bankCardPattern = #"\d{16,19}"#
    private static let emailPattern = #"[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,64}"#
    private static let ipPattern = #"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b"#
    private static let platePattern = #"[京津沪渝冀豫云辽黑湘皖鲁新苏浙赣鄂桂甘晋蒙陕吉闽贵粤青藏川宁琼使领A-Z]{1}[A-Z]{1}[A-Z0-9]{4,5}[A-Z0-9挂学警港澳]{1}"#

    // 触发脱敏的关键词
    private static let defaultKeywords = ["银行", "卡号", "账号", "身份证", "手机", "手机号", "电话", "姓名", "密码", "用户名", "邮箱", "email", "地址", "住址"]

    static func detect(in observations: [VNRecognizedTextObservation], imageSize: CGSize, customKeywords: [String]? = nil) -> [RedactionTarget] {
        var targets: [RedactionTarget] = []
        let keywords = customKeywords ?? defaultKeywords

        // 拼接所有文本进行 NLP 实体识别
        let fullText = observations.compactMap { $0.topCandidates(1).first?.string }.joined(separator: "\n")
        let nlpNames = detectNamesWithNLP(in: fullText)

        for (index, observation) in observations.enumerated() {
            guard let candidate = observation.topCandidates(1).first else { continue }
            let text = candidate.string

            // 模式 A: 直接匹配敏感模式 (身份证、手机、银行卡、邮箱、IP、车牌)
            if matches(text, pattern: idCardPattern) ||
               matches(text, pattern: phonePattern) ||
               matches(text, pattern: bankCardPattern) ||
               matches(text, pattern: emailPattern) ||
               matches(text, pattern: ipPattern) ||
               matches(text, pattern: platePattern) {
                targets.append(RedactionTarget(rect: convert(observation.boundingBox, to: imageSize), label: "PatternMatch"))
                continue
            }

            // 模式 B: NLP 实体匹配 (识别出的姓名)
            for name in nlpNames {
                if text.contains(name) && name.count >= 2 {
                    targets.append(RedactionTarget(rect: convert(observation.boundingBox, to: imageSize), label: "NLP-Name"))
                }
            }

            // 模式 C: 关键词触发 (针对你说的：标签+值)
            for keyword in keywords {
                if text.contains(keyword) {
                    // 1. 如果当前行内包含冒号或空格后的内容，且不是关键词本身
                    // 例如 "用户名: adolf" -> 匹配 "adolf"
                    let parts = text.components(separatedBy: CharacterSet(charactersIn: ":： "))
                    if parts.count > 1 && !parts.last!.isEmpty && parts.last! != keyword {
                         targets.append(RedactionTarget(rect: convert(observation.boundingBox, to: imageSize), label: "KeywordInline"))
                    }
                    // 2. 检查逻辑上的“下一个”文本块（通常是右侧或下方的数值/字符串）
                    else if index + 1 < observations.count {
                        let nextObs = observations[index + 1]
                        // 只要下一个块不是空的，就进行脱敏（针对密码、用户名等）
                        targets.append(RedactionTarget(rect: convert(nextObs.boundingBox, to: imageSize), label: "KeywordNext"))
                    }
                }
            }
        }
        return targets
    }

    /// 使用 Apple NaturalLanguage 框架识别姓名
    private static func detectNamesWithNLP(in text: String) -> [String] {
        let tagger = NLTagger(tagSchemes: [.nameType])
        tagger.string = text
        var names: [String] = []

        let options: NLTagger.Options = [.omitPunctuation, .omitWhitespace, .joinNames]
        tagger.enumerateTags(in: text.startIndex..<text.endIndex, unit: .word, scheme: .nameType, options: options) { tag, range in
            if tag == .personalName {
                let name = String(text[range])
                if name.count >= 2 { // 过滤掉单字识别错误
                    names.append(name)
                }
            }
            return true
        }
        return Array(Set(names)) // 去重
    }

    private static func matches(_ text: String, pattern: String) -> Bool {
        return text.range(of: pattern, options: .regularExpression) != nil
    }

    private static func convert(_ normalizedRect: CGRect, to size: CGSize) -> CGRect {
        // Vision 坐标系：原点 (0,0) 在左下角，Y 轴向上
        // 目标图像缓冲区坐标系：原点 (0,0) 在左上角，Y 轴向下
        //
        // 转换公式：
        // x_pixel = normalized_x * width
        // y_pixel = (1.0 - normalized_y - normalized_height) * height
        return CGRect(
            x: normalizedRect.origin.x * size.width,
            y: (1.0 - normalizedRect.origin.y - normalizedRect.size.height) * size.height,
            width: normalizedRect.size.width * size.width,
            height: normalizedRect.size.height * size.height
        )
    }

    /// 对文本内容进行脱敏处理 (用 * 替换敏感部分)
    static func redactText(_ text: String, keywords: [String] = []) -> String {
        var redacted = text
        let patterns = [idCardPattern, phonePattern, bankCardPattern, emailPattern]

        // 1. 基于正则模式脱敏 (身份证、手机号等)
        for pattern in patterns {
            if let regex = try? NSRegularExpression(pattern: pattern, options: []) {
                let range = NSRange(redacted.startIndex..<redacted.endIndex, in: redacted)
                let matches = regex.matches(in: redacted, options: [], range: range)

                for match in matches.reversed() {
                    if let textRange = Range(match.range, in: redacted) {
                        let replacement = String(repeating: "*", count: match.range.length)
                        redacted.replaceSubrange(textRange, with: replacement)
                    }
                }
            }
        }

        // 2. 基于 NLP 识别的姓名脱敏
        let nlpNames = detectNamesWithNLP(in: redacted)
        for name in nlpNames {
            redacted = redacted.replacingOccurrences(of: name, with: String(repeating: "*", count: name.count))
        }

        // 3. 基于关键词脱敏 (处理 "密码: xxxxx" 这种场景)
        let kwList = keywords.isEmpty ? defaultKeywords : keywords
        for keyword in kwList {
            let pattern = "(?i)\(keyword)[:：\\s]+([^\\s\\n，。！,;；]+)"
            if let regex = try? NSRegularExpression(pattern: pattern, options: []) {
                let range = NSRange(redacted.startIndex..<redacted.endIndex, in: redacted)
                let matches = regex.matches(in: redacted, options: [], range: range)

                for match in matches.reversed() {
                    // 我们只脱敏捕获组 1 (即关键词后面的内容)
                    if match.numberOfRanges > 1 {
                        let captureRange = match.range(at: 1)
                        if let textRange = Range(captureRange, in: redacted) {
                            let replacement = String(repeating: "*", count: captureRange.length)
                            redacted.replaceSubrange(textRange, with: replacement)
                        }
                    }
                }
            }
        }

        return redacted
    }
}
