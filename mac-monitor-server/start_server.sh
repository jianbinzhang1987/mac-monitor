#!/bin/bash

# Mac Monitor Server 启动脚本
# 同时启动后端 (Spring Boot) 和前端 (Vue)
# 控制台显示后端日志

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 存储后台进程 PID
FRONTEND_PID=""

# 清理函数 - 在脚本退出时调用
cleanup() {
    echo ""
    echo -e "${YELLOW}🛑 正在停止所有服务...${NC}"
    
    # 停止前端
    if [ -n "$FRONTEND_PID" ] && ps -p $FRONTEND_PID > /dev/null 2>&1; then
        echo -e "   停止前端服务 (PID: $FRONTEND_PID)..."
        kill $FRONTEND_PID 2>/dev/null
    fi
    
    # 额外清理：杀死所有相关的 Java 进程 (RuoYi)
    pkill -f "ruoyi-admin" 2>/dev/null
    
    # 释放端口 8080
    lsof -ti:8080 | xargs kill -9 2>/dev/null
    
    echo -e "${GREEN}✅ 所有服务已停止${NC}"
    exit 0
}

# 捕获 SIGINT (Ctrl+C) 和 SIGTERM 信号
trap cleanup SIGINT SIGTERM

echo -e "${GREEN}🚀 启动 Mac Monitor 服务端...${NC}"

# 检查端口 8080 是否被占用
if lsof -i :8080 > /dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  端口 8080 已被占用，正在尝试释放...${NC}"
    lsof -ti:8080 | xargs kill -9 2>/dev/null
    sleep 1
fi

# 检查并启动 Redis
check_redis() {
    redis-cli ping > /dev/null 2>&1 && return 0
    return 1
}

if ! check_redis; then
    echo -e "${YELLOW}⚠️  Redis 未运行，正在启动...${NC}"
    if command -v brew &> /dev/null; then
        brew services start redis > /dev/null 2>&1
    else
        redis-server --daemonize yes > /dev/null 2>&1
    fi
    
    for i in {1..10}; do
        if check_redis; then
            echo -e "${GREEN}   ✅ Redis 已启动${NC}"
            break
        fi
        sleep 1
    done
    
    if ! check_redis; then
        echo -e "${RED}   ❌ Redis 启动失败，请手动启动${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}   ✅ Redis 已在运行${NC}"
fi

# 1. 启动前端 (Vue) - 后台运行
echo -e "${GREEN}🎨 [1/2] 启动前端服务 (Vue) [后台]...${NC}"
cd "$SCRIPT_DIR/ruoyi-ui"

if [ ! -d "node_modules" ]; then
    echo -e "${YELLOW}   首次运行，正在安装依赖...${NC}"
    npm install
fi

npm run dev > /tmp/ruoyi-frontend.log 2>&1 &
FRONTEND_PID=$!
echo -e "${GREEN}   前端服务后台启动 (PID: $FRONTEND_PID)${NC}"
echo -e "${GREEN}   前端日志: tail -f /tmp/ruoyi-frontend.log${NC}"

# 2. 启动后端 (Spring Boot) - 前台运行，显示日志
echo -e "${GREEN}📦 [2/2] 启动后端服务 (Spring Boot) [前台]...${NC}"
cd "$SCRIPT_DIR"

# spring-boot:run 会自动进行增量编译

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  后端地址: http://localhost:8080${NC}"
echo -e "${GREEN}  前端地址: http://localhost:1024${NC}"
echo -e "${GREEN}  按 Ctrl+C 停止所有服务${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# 前台运行后端，直接显示日志
mvn -pl ruoyi-admin spring-boot:run -DskipTests
