# 🚨 立即修复 AuditService 连接问题

## 问题原因
Socket 文件 `/tmp/mac_monitor_audit.sock` 被 root 用户占用，需要手动清理。

## 立即执行（在终端中复制粘贴）

**一键修复命令：**
```bash
pkill -f AuditService; sudo rm -f /tmp/mac_monitor_audit.sock && cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin && ./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 & sleep 3 && lsof /tmp/mac_monitor_audit.sock && echo "✅ AuditService 已成功启动！"
```

会提示输入密码，输入后服务将自动启动。

---

## 验证服务是否正常

```bash
# 检查进程
ps aux | grep AuditService | grep -v grep

# 检查 socket
ls -la /tmp/mac_monitor_audit.sock

# 检查监听状态
lsof /tmp/mac_monitor_audit.sock

# 测试连接
echo '{"command":"get_pops","payload":null}' | nc -U /tmp/mac_monitor_audit.sock
```

---

## 如果还是失败

分步执行：

```bash
# 第 1 步：停止服务
pkill -f AuditService

# 第 2 步：删除 socket（输入密码）
sudo rm -f /tmp/mac_monitor_audit.sock

# 第 3 步：验证删除成功
ls -la /tmp/mac_monitor_audit.sock
# 应该显示: No such file or directory

# 第 4 步：启动服务
cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin
./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 &

# 第 5 步：等待 3 秒
sleep 3

# 第 6 步：验证
lsof /tmp/mac_monitor_audit.sock
```

如果第 6 步显示 AuditService 进程监听 socket，说明成功！

---

## 成功后

重新启动 GUI 应用，注册功能应该可以正常使用了。

查看日志：
```bash
# Swift/stderr 日志
tail -f /tmp/audit_service.log

# Rust 核心日志
tail -f /tmp/mac_monitor_audit_service.log
```
