<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import {
  GlobalOutlined,
  SyncOutlined,
  InfoCircleOutlined,
  LogoutOutlined,
  SearchOutlined
} from '@ant-design/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import { message } from 'ant-design-vue';
import { useNavigation } from '../composables/useNavigation';

// Internal Components
import Overview from './Overview.vue';
import TrafficAudit from './TrafficAudit.vue';
import ProcessProtection from './ProcessProtection.vue';
import FileSecurity from './FileSecurity.vue';
import ScreenshotRecord from './ScreenshotRecord.vue';
import ClipboardAudit from './ClipboardAudit.vue';
import DeviceManagement from './DeviceManagement.vue';

const { activeView } = useNavigation();

const viewNames: Record<string, string> = {
  'overview': '系统状态概览',
  'traffic': '网络上网审计',
  'process': '系统进程防护',
  'file': '文件安全监控',
  'screen': '界面行为审计',
  'clipboard': '剪贴板审计',
  'sync': '终端设备管理'
};

const currentViewName = computed(() => viewNames[activeView.value] || activeView.value);

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
    message.success('已切换至加密出口：' + value);
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
  <div class="flex flex-col h-full w-full">
    <!-- Sub-Header / ToolBar -->
    <header
      class="h-14 border-b border-macos-border flex items-center justify-between px-6 shrink-0 bg-white/30 dark:bg-black/10 backdrop-blur-md sticky top-0 z-40">
      <div class="flex items-center gap-4">
        <h2 class="text-lg font-bold tracking-tight text-macos-text">
          {{ currentViewName }}
        </h2>

        <div v-if="systemStatus.serviceRunning"
          class="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-green-500/10 text-[10px] font-bold text-green-600 dark:text-green-400 border border-green-500/20 uppercase tracking-wide">
          <span class="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse"></span>
          服务运行中
        </div>
      </div>

      <div class="flex items-center gap-3">
        <!-- Search Bar Shorthand -->
        <div class="relative group hidden sm:block">
          <SearchOutlined class="absolute left-2.5 top-1/2 -translate-y-1/2 text-macos-text-secondary w-3 h-3" />
          <input type="text" placeholder="搜索审计日志..."
            class="h-7 w-32 focus:w-48 transition-all duration-300 bg-black/5 dark:bg-white/10 border-none rounded-md pl-8 pr-3 text-xs focus:ring-1 focus:ring-macos-accent outline-none" />
        </div>

        <a-divider type="vertical" />

        <div class="flex items-center gap-1 bg-black/5 dark:bg-white/10 rounded-md px-2 py-1 h-7">
          <GlobalOutlined class="text-[10px] text-macos-text-secondary" />
          <a-select v-model:value="currentPop" class="pop-select" :loading="loadingPop" @change="handlePopChange"
            :bordered="false" size="small">
            <a-select-option v-for="node in popNodes" :key="node.id" :value="node.id">
              {{ node.name }}
            </a-select-option>
          </a-select>
        </div>

        <button @click="handleCheckUpdate"
          class="p-1.5 rounded-md hover:bg-black/5 dark:hover:bg-white/10 text-macos-text-secondary transition-colors">
          <InfoCircleOutlined class="text-sm" />
        </button>

        <button class="p-1.5 rounded-md hover:bg-red-500/10 text-red-500 transition-colors">
          <LogoutOutlined class="text-sm" />
        </button>
      </div>
    </header>

    <!-- Main View Content -->
    <div class="flex-1 overflow-auto">
      <transition name="fade" mode="out-in">
        <div :key="activeView" class="p-6">
          <Overview v-if="activeView === 'overview'" />
          <TrafficAudit v-else-if="activeView === 'traffic'" />
          <ProcessProtection v-else-if="activeView === 'process'" />
          <FileSecurity v-else-if="activeView === 'file'" />
          <ScreenshotRecord v-else-if="activeView === 'screen'" />
          <ClipboardAudit v-else-if="activeView === 'clipboard'" />
          <DeviceManagement v-else-if="activeView === 'sync'" />
          <div v-else class="flex flex-col items-center justify-center p-12 text-macos-text-secondary opacity-50">
            <SyncOutlined class="text-4xl mb-4 animate-spin" />
            <p>模块加载中...</p>
          </div>
        </div>
      </transition>
    </div>
  </div>
</template>

<style scoped>
.pop-select {
  width: 90px;
}

:deep(.ant-select-selection-item) {
  font-size: 11px !important;
  font-weight: 600 !important;
  color: var(--macos-text) !important;
}

/* Page Transitions */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.fade-enter-from {
  opacity: 0;
  transform: translateY(4px);
}

.fade-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

/* Custom Scrollbar for macOS look */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 4px;
}

.dark ::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
}
</style>