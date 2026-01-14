#ifndef NETWORK_STACK_H
#define NETWORK_STACK_H

#include <stdint.h>

// 初始化 Rust 网络协议栈
// 返回 0 表示成功，非 0 表示失败
int32_t init_stack();

// 关闭并清理网络协议栈
void shutdown_stack();

// 处理入站数据包 (System -> Rust -> Internet)
// data: IP 数据包的原始字节指针
// len: 数据包长度
// 返回 0 表示处理成功
int32_t process_packet(const uint8_t *data, size_t len);

// 获取出站数据包 (Internet -> Rust -> System)
// buffer: 用于存储出站数据的缓冲区
// buffer_len: 缓冲区最大长度
// written_len: 实际写入的数据长度
// 返回 0 表示成功获取数据
int32_t get_outbound_packet(uint8_t *buffer, size_t buffer_len, size_t *written_len);

// 驱动协议栈进行轮询处理 (定时调用)
void poll_stack();

#endif /* NETWORK_STACK_H */