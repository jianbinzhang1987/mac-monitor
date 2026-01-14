#ifndef AUDIT_CORE_H
#define AUDIT_CORE_H

#include <stdint.h>
#include <stddef.h>

// 分析图像缓冲区 (OCR & 隐私脱敏)
// ptr: 图像原始数据指针 (BGRA/RGBA)
// len: 数据长度
// width: 图像宽度
// height: 图像高度
void analyze_image_buffer(const uint8_t *ptr, size_t len, uint32_t width, uint32_t height);

// 记录审计事件
// event_json: JSON 格式的审计事件字符串
void log_audit_event(const char *event_json);

#endif /* AUDIT_CORE_H */