# ✅ AuditService 连接问题解决方案

## 问题诊断

经过详细分析，发现以下问题：

1. **Socket 文件权限问题** ✅ 已修复
   - 之前的 socket 文件被 root 用户创建
   - 已更新代码，添加更好的错误提示

2. **进程持久化问题** ⚠️ 需要解决
   - AuditService 在后台启动后立即退出
   - 尽管代码中有 `CFRunLoopRun()`，但在后台模式下不稳定

## 解决方案

### 方案 A：使用脚本在新终端窗口启动（推荐）

这个方案会在独立的 Terminal 窗口中运行 AuditService，可以看到实时日志：

```bash
cd /Users/adolf/Desktop/code/clash/mac-monitor-project
./start_audit_daemon.sh
```

**优点：**
- ✅ 可以实时看到日志输出
- ✅ 进程稳定运行
- ✅ 容易调试

### 方案 B：手动在终端中启动（最简单）

直接在终端运行：

```bash
cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin
./AuditService.app/Contents/MacOS/AuditService
```

**让终端窗口保持打开**，AuditService 就会持续运行。

验证连接：
```bash
# 在另一个终端窗口中测试
echo '{"command":"get_pops","payload":null}' | nc -U /tmp/mac_monitor_audit.sock
```

### 方案 C：使用 launchd 创建系统服务（生产环境）

创建 plist 文件：

```bash
cat > ~/Library/LaunchAgents/com.mac-monitor.audit-service.plist <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.mac-monitor.audit-service</string>

    <key>ProgramArguments</key>
    <array>
        <string>/Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin/AuditService.app/Contents/MacOS/AuditService</string>
    </array>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <true/>

    <key>StandardOutPath</key>
    <string>/tmp/audit_service.log</string>

    <key>StandardErrorPath</key>
    <string>/tmp/audit_service.log</string>

    <key>WorkingDirectory</key>
    <string>/Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin</string>
</dict>
</plist>
EOF

# 加载服务
launchctl load ~/Library/LaunchAgents/com.mac-monitor.audit-service.plist

# 启动服务
launchctl start com.mac-monitor.audit-service

# 查看状态
launchctl list | grep audit
```

卸载服务：
```bash
launchctl unload ~/Library/LaunchAgents/com.mac-monitor.audit-service.plist
rm ~/Library/LaunchAgents/com.mac-monitor.audit-service.plist
```

## 验证服务正常运行

运行以下命令验证：

```bash
# 1. 检查进程
ps aux | grep AuditService | grep -v grep

# 2. 检查 socket
ls -la /tmp/mac_monitor_audit.sock

# 3. 检查监听
lsof /tmp/mac_monitor_audit.sock

# 4. 测试连接
echo '{"command":"get_pops","payload":null}' | nc -U /tmp/mac_monitor_audit.sock
```

如果第 4 步返回类似以下内容，说明成功：
```json
{"status":"ok","message":"Success","payload":[...]}
```

## 现在该做什么？

### 立即修复步骤：

**选择方案 B（最快）：**

1. 打开终端
2. 运行：
   ```bash
   cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin
   ./AuditService.app/Contents/MacOS/AuditService
   ```
3. 保持这个终端窗口打开
4. 启动 GUI 应用并测试注册功能

### 验证成功：

在另一个终端中运行：
```bash
echo '{"command":"get_pops","payload":null}' | nc -U /tmp/mac_monitor_audit.sock
```

如果返回 JSON 响应，说明 AuditService 正常工作！

## 后续改进建议

1. **修复后台运行问题**
   - 调查为什么 `CFRunLoopRun()` 在后台模式下不工作
   - 可能需要添加 `DispatchQueue` 或其他 RunLoop 设置

2. **使用 launchd**
   - 生产环境应该使用方案 C
   - 可以自动重启、开机启动

3. **改进日志**
   - 添加更详细的启动日志
   - 记录进程为何退出的原因

## 常见问题

### Q: 为什么后台启动会失败？
A: Swift 的 `CFRunLoopRun()` 需要主线程的 RunLoop 处于活动状态。在后台启动时，可能没有正确初始化 RunLoop。

### Q: 如何确保服务一直运行？
A: 使用 launchd（方案 C）或者在独立终端窗口运行（方案 A/B）。

### Q: Socket 权限问题如何彻底解决？
A: 已在代码中添加自动清理和权限设置。如果仍有问题，运行：
```bash
sudo rm -f /tmp/mac_monitor_audit.sock
```

---

**现在请选择一个方案启动 AuditService！**

推荐：先用方案 B 快速验证，成功后再配置方案 C 用于长期运行。
