#!/bin/bash

# 测试审计服务的响应

echo "Testing AuditService IPC..."
echo ""

# 测试注册命令
echo "1. Testing register command:"
echo '{"command":"register","payload":{"server_ip":"127.0.0.1","server_port":"8080","cpe_id":"test123","pin":"123456"}}' | nc -U /tmp/mac_monitor_audit.sock
echo ""
echo ""

# 等待一下
sleep 1

# 测试登录命令
echo "2. Testing login command:"
echo '{"command":"login","payload":{"pin":"123456"}}' | nc -U /tmp/mac_monitor_audit.sock
echo ""
echo ""

sleep 1

# 测试get_pops命令
echo "3. Testing get_pops command:"
echo '{"command":"get_pops","payload":null}' | nc -U /tmp/mac_monitor_audit.sock
echo ""
echo ""

echo "Test completed!"
