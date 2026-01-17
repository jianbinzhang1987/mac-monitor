<script setup lang="ts">
import { ref, onMounted } from 'vue';
import {
  PieChartOutlined,
  DesktopOutlined,
  UserOutlined,
  FileOutlined,
  SafetyCertificateOutlined,
  CameraOutlined,
  GlobalOutlined,
  SyncOutlined,
  InfoCircleOutlined,
  LogoutOutlined,
  CopyOutlined,
} from '@ant-design/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import { message } from 'ant-design-vue';
import Overview from './Overview.vue';
import TrafficAudit from './TrafficAudit.vue';
import ProcessProtection from './ProcessProtection.vue';
import FileSecurity from './FileSecurity.vue';
import ScreenshotRecord from './ScreenshotRecord.vue';
import ClipboardAudit from './ClipboardAudit.vue';


const collapsed = ref<boolean>(false);
const selectedKeys = ref<string[]>(['1']);

type PopNode = { id: string; name: string; latency: number };
const popNodes = ref<PopNode[]>([]);
const currentPop = ref<string>('hk-01');
const loadingPop = ref(false);

const systemStatus = ref({
  serviceRunning: true,
  auditCount: 128,
  riskCount: 2,
  lastSync: '10:23',
});

onMounted(async () => {
  try {
    popNodes.value = await invoke('get_pop_nodes');
  } catch (err) {
    console.error('Failed to fetch pops:', err);
  }
});

const handlePopChange = async (value: string) => {
  loadingPop.value = true;
  try {
    await invoke('switch_pop_node', { nodeId: value });
    currentPop.value = value;
    message.success('已连接至新节点');
  } catch (err) {
    message.error('切换失败');
  } finally {
    loadingPop.value = false;
  }
};

const handleCheckUpdate = async () => {
  try {
    const res = await invoke('check_for_updates') as any;
    if (res.has_update) {
      message.info('发现新版本: ' + res.latest_version);
    } else {
      message.success('当前已是最新版本');
    }
  } catch {
    message.error('检查更新失败');
  }
};
</script>

<template>
  <div class="premium-dashboard">
    <a-layout style="min-height: 100vh; background: transparent">
      <!-- 侧边栏 -->
      <a-layout-sider v-model:collapsed="collapsed" collapsible class="glass-sider">
        <div class="logo-area">
          <div class="logo-icon">M</div>
          <span v-if="!collapsed" class="logo-text">Mac Monitor</span>
        </div>
        <a-menu v-model:selectedKeys="selectedKeys" mode="inline" class="premium-menu">
          <a-menu-item key="1">
            <pie-chart-outlined />
            <span>系统概览</span>
          </a-menu-item>
          <a-menu-item key="2">
            <desktop-outlined />
            <span>网络审计</span>
          </a-menu-item>
          <a-sub-menu key="sub1">
            <template #title>
              <span>
                <user-outlined />
                <span>行为日志</span>
              </span>
            </template>
            <a-menu-item key="3"><safety-certificate-outlined /> 进程防护</a-menu-item>
            <a-menu-item key="4"><file-outlined /> 文件安全</a-menu-item>
            <a-menu-item key="5"><camera-outlined /> 截屏记录</a-menu-item>
            <a-menu-item key="7"><copy-outlined /> 剪贴板审计</a-menu-item>
          </a-sub-menu>
          <a-menu-item key="6">
            <sync-outlined />
            <span>同步状态</span>
          </a-menu-item>
        </a-menu>
      </a-layout-sider>

      <a-layout class="main-layout">
        <!-- 顶部状态栏 -->
        <a-layout-header class="glass-header">
          <div class="header-left">
            <div class="status-indicator">
              <span class="dot pulse"></span>
              <span class="status-text">审计引擎运行中 (抢占模式)</span>
            </div>
          </div>
          <div class="header-right">
            <GlobalOutlined class="pop-icon" />
            <a-select v-model:value="currentPop" class="pop-select" :loading="loadingPop" @change="handlePopChange"
              :bordered="false">
              <a-select-option v-for="node in popNodes" :key="node.id" :value="node.id">
                {{ node.name }} ({{ node.latency }}ms)
              </a-select-option>
            </a-select>
            <a-divider type="vertical" />
            <a-tooltip title="版本信息">
              <info-circle-outlined class="header-action-icon" @click="handleCheckUpdate" />
            </a-tooltip>
            <a-button type="text" class="logout-btn">
              <logout-outlined />
            </a-button>
          </div>
        </a-layout-header>

        <!-- 内容区 -->
        <a-layout-content class="content-wrapper">
          <Overview v-if="selectedKeys[0] === '1'" />
          <TrafficAudit v-else-if="selectedKeys[0] === '2'" />
          <ProcessProtection v-else-if="selectedKeys[0] === '3'" />
          <FileSecurity v-else-if="selectedKeys[0] === '4'" />
          <ScreenshotRecord v-else-if="selectedKeys[0] === '5'" />
          <ClipboardAudit v-else-if="selectedKeys[0] === '7'" />
          <div v-else class="coming-soon">
            <a-empty description="功能模块开发中" />
          </div>
        </a-layout-content>
      </a-layout>
    </a-layout>
  </div>
</template>

<style scoped>
.premium-dashboard {
  background: linear-gradient(135deg, #0f172a 0%, #1e1b4b 100%);
  color: #fff;
  font-family: 'Inter', -apple-system, sans-serif;
}

/* 侧边栏样式 */
.glass-sider {
  background: rgba(255, 255, 255, 0.03) !important;
  backdrop-filter: blur(10px);
  border-right: 1px solid rgba(255, 255, 255, 0.05);
}

.logo-area {
  height: 64px;
  display: flex;
  align-items: center;
  padding: 0 20px;
  gap: 12px;
}

.logo-icon {
  width: 32px;
  height: 32px;
  background: linear-gradient(to bottom right, #3b82f6, #8b5cf6);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
}

.logo-text {
  font-size: 18px;
  font-weight: 700;
  letter-spacing: -0.5px;
}

.premium-menu {
  background: transparent !important;
  border-right: none !important;
}

:deep(.ant-menu-item),
:deep(.ant-menu-submenu-title) {
  color: #94a3b8 !important;
  margin: 4px 8px !important;
  border-radius: 8px !important;
}

:deep(.ant-menu-item-selected) {
  background: rgba(59, 130, 246, 0.1) !important;
  color: #3b82f6 !important;
}

/* Header 样式 */
.glass-header {
  background: rgba(15, 23, 42, 0.5) !important;
  backdrop-filter: blur(12px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  height: 64px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 24px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 10px;
  background: rgba(255, 255, 255, 0.05);
  padding: 6px 12px;
  border-radius: 20px;
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #10b981;
}

.pulse {
  box-shadow: 0 0 0 rgba(16, 185, 129, 0.4);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% {
    box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.7);
  }

  70% {
    box-shadow: 0 0 0 10px rgba(16, 185, 129, 0);
  }

  100% {
    box-shadow: 0 0 0 0 rgba(16, 185, 129, 0);
  }
}

.status-text {
  font-size: 13px;
  color: #88fa88;
  font-weight: 500;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.pop-icon {
  font-size: 16px;
  color: #94a3b8;
}

.pop-select {
  width: 150px;
  color: #fff !important;
}

:deep(.ant-select-selector) {
  color: #fff !important;
}

.header-action-icon {
  font-size: 18px;
  color: #94a3b8;
  cursor: pointer;
  transition: color 0.3s;
}

.header-action-icon:hover {
  color: #fff;
}

.logout-btn {
  color: #f43f5e;
}

/* 内容区样式 */
.content-wrapper {
  padding: 24px 32px;
}

.page-header {
  margin-bottom: 24px;
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
}

.page-title {
  font-size: 24px;
  font-weight: 700;
  margin: 0;
  color: #f8fafc;
}

.date-time {
  color: #64748b;
  font-size: 13px;
}

.card-glass {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 16px;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s, background 0.3s;
}

.card-glass:hover {
  background: rgba(255, 255, 255, 0.05);
  transform: translateY(-2px);
}

.stat-card {
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon-wrap {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
}

.stat-icon-wrap.blue {
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

.stat-icon-wrap.red {
  background: rgba(244, 63, 94, 0.1);
  color: #f43f5e;
}

.stat-icon-wrap.green {
  background: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.stat-icon-wrap.orange {
  background: rgba(245, 158, 11, 0.1);
  color: #f59e0b;
}

.stat-label {
  font-size: 13px;
  color: #94a3b8;
}

.stat-value {
  font-size: 22px;
  font-weight: 700;
}

.stat-value .unit {
  font-size: 12px;
  font-weight: normal;
  color: #64748b;
  margin-left: 2px;
}

.main-card {
  padding: 24px;
  min-height: 400px;
}

.mt-24 {
  margin-top: 24px;
}

:deep(.ant-tabs-tab) {
  color: #94a3b8 !important;
}

:deep(.ant-tabs-tab-active) {
  color: #3b82f6 !important;
}

.chart-placeholder {
  height: 300px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #475569;
}

.mock-line {
  width: 100%;
  height: 2px;
  background: linear-gradient(to right, transparent, #3b82f6, transparent);
  margin-bottom: 20px;
}

.coming-soon {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 400px;
}
</style>