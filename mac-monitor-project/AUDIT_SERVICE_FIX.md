# 审计服务连接问题修复指南

## 问题描述
GUI 应用启动时报错：`注册失败: 无法连接到审计服务: Connection refused (os error 61)`

## 根本原因
1. AuditService 的 IPC Unix socket (`/tmp/mac_monitor_audit.sock`) 被 root 用户创建
2. 当前用户无法删除该 socket 文件
3. 导致新启动的 AuditService 无法绑定 socket（Address already in use）

## 解决方案

### 方案 1: 手动清理并重启（推荐）

在终端中运行以下命令：

```bash
# 停止所有 AuditService 进程
pkill -f AuditService

# 删除旧的 socket 文件（需要输入密码）
sudo rm -f /tmp/mac_monitor_audit.sock

# 启动 AuditService
cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin
./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 &

# 等待 3 秒后验证
sleep 3
lsof /tmp/mac_monitor_audit.sock
```

**一键执行版本：**
```bash
pkill -f AuditService; sudo rm -f /tmp/mac_monitor_audit.sock && cd /Users/adolf/Desktop/code/clash/mac-monitor-project/gui-app/src-tauri/bin && ./AuditService.app/Contents/MacOS/AuditService > /tmp/audit_service.log 2>&1 & sleep 3 && echo "✅ Service started!" && lsof /tmp/mac_monitor_audit.sock
```

### 方案 2: 使用提供的脚本

我已经创建了 `cleanup_and_start.sh` 脚本，但它需要交互式输入 sudo 密码。

在终端中运行：
```bash
cd /Users/adolf/Desktop/code/clash/mac-monitor-project
./cleanup_and_start.sh
```

## 验证服务正常运行

执行以下命令验证：

```bash
# 1. 检查进程是否运行
ps aux | grep AuditService | grep -v grep

# 2. 检查 socket 文件
ls -la /tmp/mac_monitor_audit.sock

# 3. 检查是否有进程监听 socket
lsof /tmp/mac_monitor_audit.sock

# 4. 查看服务日志
tail -f /tmp/audit_service.log

# 5. 查看 Rust 核心日志
tail -f /tmp/mac_monitor_audit_service.log
```

## 测试连接

使用 `nc` 命令测试 IPC 连接：

```bash
echo '{"command":"get_pops","payload":null}' | nc -U /tmp/mac_monitor_audit.sock
```

应该返回类似：
```json
{"status":"ok","message":"Success","payload":[...]}
```

## 预防措施

为避免此问题再次发生，建议修改代码使 socket 文件不以 root 权限创建。

### 修改 IPC 服务器代码

在 `audit-service/rust-core/src/ipc/mod.rs` 中，socket 创建后设置权限：

```rust
// 第 40-51 行附近
let listener = LocalSocketListener::bind(name)
    .expect("Failed to bind IPC socket");

// 设置权限为 777（所有用户可读写）
use std::os::unix::fs::PermissionsExt;
if let Ok(metadata) = std::fs::metadata(name) {
    let mut perms = metadata.permissions();
    perms.set_mode(0o777);
    if let Err(e) = std::fs::set_permissions(name, perms) {
        eprintln!("Failed to set IPC socket permissions: {}", e);
    }
}
```

**注意：** 这段代码已经存在于代码中（第 44-51 行），但如果服务以 root 权限运行，socket 仍会被 root 拥有。

## 长期解决方案

1. **不要以 root 权限运行 AuditService**
   - 当前 AuditService 应该以普通用户权限运行
   - 只有需要 EndpointSecurity 权限时才需要 root

2. **在服务启动时清理旧 socket**
   - 在 `lib.rs` 的 `init_service_context()` 中添加：
   ```rust
   // 启动 IPC 服务前清理旧 socket
   let _ = std::fs::remove_file("/tmp/mac_monitor_audit.sock");
   ```

3. **使用用户特定的 socket 路径**
   - 改用 `~/Library/Application Support/Mac Monitor/audit.sock`
   - 避免 `/tmp` 目录的权限问题

## 故障排查

### 如果服务启动失败

1. 查看 Swift 日志：
   ```bash
   tail -50 /tmp/audit_service.log
   ```

2. 查看 Rust 核心日志：
   ```bash
   tail -50 /tmp/mac_monitor_audit_service.log
   ```

3. 检查是否有 panic：
   ```bash
   grep -i panic /tmp/audit_service.log
   ```

### 如果 socket 连接失败

1. 确认 socket 文件存在且有正确权限：
   ```bash
   ls -la /tmp/mac_monitor_audit.sock
   ```
   应该显示：`srwxrwxrwx`

2. 确认有进程监听：
   ```bash
   lsof /tmp/mac_monitor_audit.sock
   ```
   应该显示 AuditService 进程

3. 手动测试连接：
   ```bash
   echo '{"command":"get_pops","payload":null}' | nc -U /tmp/mac_monitor_audit.sock
   ```

## 当前状态

根据最新检查：
- ✅ AuditService 二进制文件存在
- ✅ Socket 文件存在但权限有问题（owned by root）
- ❌ IPC 服务器无法绑定 socket（Address already in use）
- ⚠️ 服务部分功能正常（截屏、剪贴板监控）但 IPC 不可用

**下一步：** 请在终端运行方案 1 的命令来修复此问题。
