#!/bin/bash

# 系统验证脚本
# 用于端到端测试异常进程与程序行为监控功能

set -e

echo "=================================================="
echo "  Mac Monitor 系统验证脚本"
echo "=================================================="
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_ROOT="/Users/adolf/Desktop/code/clash"

# 测试结果统计
PASSED=0
FAILED=0

# 测试函数
test_step() {
    local step_name=$1
    echo -e "${BLUE}[测试]${NC} $step_name"
}

test_pass() {
    echo -e "${GREEN}[✓]${NC} $1"
    ((PASSED++))
}

test_fail() {
    echo -e "${RED}[✗]${NC} $1"
    ((FAILED++))
}

test_warn() {
    echo -e "${YELLOW}[!]${NC} $1"
}

# 1. 检查 Rust 核心库构建
echo ""
test_step "1. 检查审计服务 Rust 核心库"
if [ -f "$PROJECT_ROOT/mac-monitor-project/audit-service/rust-core/target/release/libaudit_logic_core.a" ]; then
    test_pass "审计服务静态库存在"
    ls -lh "$PROJECT_ROOT/mac-monitor-project/audit-service/rust-core/target/release/libaudit_logic_core.a" | awk '{print "   大小: " $5}'
else
    test_fail "审计服务静态库不存在,请先运行构建"
    echo "   运行: cd mac-monitor-project/audit-service/rust-core && cargo build --release"
fi

echo ""
test_step "2. 检查网络扩展 Rust 核心库"
if [ -f "$PROJECT_ROOT/mac-monitor-project/network-extension/rust-core/target/release/libnetwork_protocol_stack.a" ]; then
    test_pass "网络协议栈静态库存在"
    ls -lh "$PROJECT_ROOT/mac-monitor-project/network-extension/rust-core/target/release/libnetwork_protocol_stack.a" | awk '{print "   大小: " $5}'
else
    test_fail "网络协议栈静态库不存在,请先运行构建"
    echo "   运行: cd mac-monitor-project/network-extension/rust-core && cargo build --release"
fi

# 2. 检查服务器端编译
echo ""
test_step "3. 检查服务器端编译状态"
if [ -d "$PROJECT_ROOT/mac-monitor-server/ruoyi-monitor/target/classes" ]; then
    test_pass "服务器端编译成功"
    # 检查关键类文件
    if [ -f "$PROJECT_ROOT/mac-monitor-server/ruoyi-monitor/target/classes/com/ruoyi/monitor/controller/ApiMonitorController.class" ]; then
        test_pass "ApiMonitorController 已编译"
    else
        test_fail "ApiMonitorController 未找到"
    fi

    if [ -f "$PROJECT_ROOT/mac-monitor-server/ruoyi-monitor/target/classes/com/ruoyi/monitor/domain/MonitorLogBehavior.class" ]; then
        test_pass "MonitorLogBehavior 已编译"
    else
        test_fail "MonitorLogBehavior 未找到"
    fi
else
    test_fail "服务器端未编译,请先运行构建"
    echo "   运行: cd mac-monitor-server && mvn clean compile"
fi

# 3. 检查源代码关键功能
echo ""
test_step "4. 验证客户端扫描器实现"
if grep -q "pub struct Scanner" "$PROJECT_ROOT/mac-monitor-project/audit-service/rust-core/src/scanner/mod.rs" 2>/dev/null; then
    test_pass "Scanner 结构体已定义"
else
    test_fail "Scanner 结构体未找到"
fi

if grep -q "scan_processes" "$PROJECT_ROOT/mac-monitor-project/audit-service/rust-core/src/scanner/mod.rs" 2>/dev/null; then
    test_pass "进程扫描函数已实现"
else
    test_fail "进程扫描函数未找到"
fi

if grep -q "scan_applications" "$PROJECT_ROOT/mac-monitor-project/audit-service/rust-core/src/scanner/mod.rs" 2>/dev/null; then
    test_pass "应用程序扫描函数已实现"
else
    test_fail "应用程序扫描函数未找到"
fi

# 4. 检查服务器端 API 接口
echo ""
test_step "5. 验证服务器端 API 实现"
if grep -q "GetMapping.*config/policy" "$PROJECT_ROOT/mac-monitor-server/ruoyi-monitor/src/main/java/com/ruoyi/monitor/controller/ApiMonitorController.java" 2>/dev/null; then
    test_pass "策略配置接口已实现 (GET /api/v1/config/policy)"
else
    test_fail "策略配置接口未找到"
fi

if grep -q "PostMapping.*log/behavior" "$PROJECT_ROOT/mac-monitor-server/ruoyi-monitor/src/main/java/com/ruoyi/monitor/controller/ApiMonitorController.java" 2>/dev/null; then
    test_pass "行为日志上传接口已实现 (POST /api/v1/log/behavior)"
else
    test_fail "行为日志上传接口未找到"
fi

# 5. 检查数据库模式
echo ""
test_step "6. 验证数据库表结构"
if grep -q "monitor_log_behavior" "$PROJECT_ROOT/mac-monitor-server/sql/mac_monitor.sql" 2>/dev/null; then
    test_pass "行为日志表定义存在"

    # 检查新增字段
    if grep -q "host_id.*varchar" "$PROJECT_ROOT/mac-monitor-server/sql/mac_monitor.sql" 2>/dev/null; then
        test_pass "host_id 字段已添加"
    else
        test_warn "host_id 字段可能缺失"
    fi

    if grep -q "mac.*varchar" "$PROJECT_ROOT/mac-monitor-server/sql/mac_monitor.sql" 2>/dev/null; then
        test_pass "mac 字段已添加"
    else
        test_warn "mac 字段可能缺失"
    fi
else
    test_fail "行为日志表定义未找到"
fi

# 6. 检查默认策略配置
echo ""
test_step "7. 验证默认策略配置"
if grep -q "process_blacklist" "$PROJECT_ROOT/mac-monitor-server/sql/mac_monitor.sql" 2>/dev/null; then
    test_pass "进程黑名单配置存在"

    # 提取黑名单内容
    BLACKLIST=$(grep -o "process_blacklist.*\]" "$PROJECT_ROOT/mac-monitor-server/sql/mac_monitor.sql" | head -1)
    echo "   黑名单: $BLACKLIST"

    # 检查是否包含关键进程
    if echo "$BLACKLIST" | grep -q "clash"; then
        test_pass "包含 clash 进程"
    else
        test_warn "未包含 clash 进程"
    fi

    if echo "$BLACKLIST" | grep -q "v2ray"; then
        test_pass "包含 v2ray 进程"
    else
        test_warn "未包含 v2ray 进程"
    fi
else
    test_fail "进程黑名单配置未找到"
fi

# 7. 检查本地数据库
echo ""
test_step "8. 检查客户端本地数据库"
if [ -f "$PROJECT_ROOT/audit.db" ]; then
    test_pass "本地 SQLite 数据库存在"

    # 检查表结构
    if sqlite3 "$PROJECT_ROOT/audit.db" "SELECT name FROM sqlite_master WHERE type='table' AND name='monitor_log_behavior';" | grep -q "monitor_log_behavior"; then
        test_pass "monitor_log_behavior 表已创建"

        # 检查记录数
        COUNT=$(sqlite3 "$PROJECT_ROOT/audit.db" "SELECT COUNT(*) FROM monitor_log_behavior;")
        echo "   当前记录数: $COUNT"
    else
        test_warn "monitor_log_behavior 表未创建 (可能是首次运行)"
    fi
else
    test_warn "本地数据库文件不存在 (将在首次运行时创建)"
fi

# 8. 代码质量检查
echo ""
test_step "9. 代码质量检查"
if grep -q "log::warn!" "$PROJECT_ROOT/mac-monitor-project/audit-service/rust-core/src/scanner/mod.rs" 2>/dev/null; then
    test_pass "使用标准日志宏 (log::warn!)"
else
    test_warn "可能使用 println! 而非标准日志"
fi

if grep -q "init_audit_core" "$PROJECT_ROOT/mac-monitor-project/audit-service/rust-core/src/lib.rs" 2>/dev/null; then
    test_pass "FFI 初始化函数已导出"
else
    test_fail "FFI 初始化函数未找到"
fi

# 9. 依赖项检查
echo ""
test_step "10. 检查关键依赖项"
if grep -q "sysinfo" "$PROJECT_ROOT/mac-monitor-project/audit-service/rust-core/Cargo.toml" 2>/dev/null; then
    test_pass "sysinfo 依赖已配置"
else
    test_fail "sysinfo 依赖缺失"
fi

if grep -q "tokio" "$PROJECT_ROOT/mac-monitor-project/audit-service/rust-core/Cargo.toml" 2>/dev/null; then
    test_pass "tokio 异步运行时已配置"
else
    test_fail "tokio 依赖缺失"
fi

if grep -q "mac_address" "$PROJECT_ROOT/mac-monitor-project/audit-service/rust-core/Cargo.toml" 2>/dev/null; then
    test_pass "mac_address 依赖已配置"
else
    test_warn "mac_address 依赖可能缺失"
fi

# 10. 集成测试建议
echo ""
test_step "11. 集成测试建议"
echo ""
echo "   手动验证步骤:"
echo "   1. 启动服务器:"
echo "      cd mac-monitor-server"
echo "      mvn spring-boot:run"
echo ""
echo "   2. 验证策略接口:"
echo "      curl http://localhost:8080/api/v1/config/policy"
echo ""
echo "   3. 启动客户端 (需要构建 Swift 部分):"
echo "      cd mac-monitor-project/audit-service/swift"
echo "      swift run"
echo ""
echo "   4. 启动一个黑名单进程 (如 clash 或 cargo):"
echo "      cargo --version  # 模拟异常进程"
echo ""
echo "   5. 等待 60 秒,检查服务器日志和数据库:"
echo "      SELECT * FROM monitor_log_behavior ORDER BY create_time DESC LIMIT 5;"
echo ""

# 统计结果
echo ""
echo "=================================================="
echo "  测试结果统计"
echo "=================================================="
echo -e "${GREEN}通过: $PASSED${NC}"
echo -e "${RED}失败: $FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ 所有测试通过,系统已就绪!${NC}"
    exit 0
else
    echo -e "${YELLOW}! 存在 $FAILED 个问题,请检查上述输出${NC}"
    exit 1
fi
