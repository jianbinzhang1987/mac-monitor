<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { message } from 'ant-design-vue';
import {
  RefreshCcw,
  Download,
  Search,
  Filter,
  ChevronRight,
  ShieldCheck,
  Globe,
  Clock
} from 'lucide-vue-next';

interface TrafficLog {
  id: string;
  timestamp: string;
  protocol: string;
  src_addr: string;
  dst_addr: string;
  url: string;
  method: string;
  status: number;
  size: number;
}

const loading = ref(false);
const trafficLogs = ref<TrafficLog[]>([]);
const selectedRow = ref<TrafficLog | null>(null);

const columns = [
  { title: '审计时间', dataIndex: 'timestamp', key: 'timestamp', width: 140 },
  { title: '协议', dataIndex: 'protocol', key: 'protocol', width: 80 },
  { title: '操作方法', dataIndex: 'method', key: 'method', width: 80 },
  { title: '状态码', dataIndex: 'status', key: 'status', width: 70 },
  { title: '访问地址', dataIndex: 'url', key: 'url', ellipsis: true },
];

const loadTrafficLogs = async () => {
  loading.value = true;
  try {
    // Mock Data for now
    trafficLogs.value = Array.from({ length: 30 }, (_, i) => ({
      id: `log-${i}`,
      timestamp: new Date(Date.now() - i * 5000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' }),
      protocol: ['HTTPS', 'HTTP', 'WS'][i % 3],
      src_addr: `192.168.1.${100 + i}`,
      dst_addr: `172.67.14.${i}`,
      url: `https://api.github.com/repos/tauri-apps/tauri/commits?per_page=1&page=${i}`,
      method: ['GET', 'POST', 'PUT'][i % 3],
      status: [200, 201, 304, 404][i % 4],
      size: Math.floor(Math.random() * 50000) + 1000,
    }));
  } catch (err) {
    message.error('无法加载审计日志');
  } finally {
    loading.value = false;
  }
};

const onRowClick = (record: TrafficLog) => {
  selectedRow.value = record;
};

onMounted(loadTrafficLogs);

const formatBytes = (bytes: number) => {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
};
</script>

<template>
  <div class="h-full flex flex-col overflow-hidden animate-in fade-in duration-500">
    <!-- Toolbar -->
    <div class="flex items-center justify-between mb-4 shrink-0 px-1">
      <div class="flex items-center gap-2">
        <div class="relative">
          <Search class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-macos-text-secondary opacity-50" />
          <input type="text" placeholder="按 URL、IP 或 进程过滤..."
            class="h-8 w-64 bg-black/5 dark:bg-white/5 border border-macos-border rounded-lg pl-9 pr-3 text-xs outline-none focus:ring-1 focus:ring-macos-accent transition-all" />
        </div>
        <button
          class="flex items-center gap-1.5 px-3 h-8 bg-black/5 dark:bg-white/5 border border-macos-border rounded-lg text-xs font-semibold text-macos-text-secondary hover:bg-black/10 transition-colors">
          <Filter class="w-3 h-3" />
          高级筛选
        </button>
      </div>

      <div class="flex items-center gap-2">
        <button @click="loadTrafficLogs"
          class="p-2 hover:bg-black/5 dark:hover:bg-white/10 rounded-lg transition-colors"
          :class="{ 'animate-spin': loading }">
          <RefreshCcw class="w-4 h-4 text-macos-text-secondary" />
        </button>
        <button
          class="flex items-center gap-2 px-3 h-8 bg-macos-accent text-white rounded-lg text-xs font-bold shadow-sm hover:bg-macos-accent-hover transition-colors">
          <Download class="w-3.5 h-3.5" />
          报告导出
        </button>
      </div>
    </div>

    <!-- Main Content: Master-Detail -->
    <div class="flex-1 flex gap-4 overflow-hidden">
      <!-- Table View -->
      <div
        class="flex-1 overflow-hidden bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl shadow-sm flex flex-col">
        <div class="flex-1 overflow-auto">
          <a-table :columns="columns" :data-source="trafficLogs" :loading="loading" :pagination="false" row-key="id"
            size="small" :custom-row="(record) => ({ onClick: () => onRowClick(record) })" class="macos-table">
            <template #bodyCell="{ column, record }">
              <template v-if="column.key === 'protocol'">
                <span class="px-2 py-0.5 rounded text-[10px] font-bold tracking-tight uppercase"
                  :class="record.protocol === 'HTTPS' ? 'bg-green-500/10 text-green-600' : 'bg-blue-500/10 text-blue-600'">
                  {{ record.protocol }}
                </span>
              </template>
              <template v-if="column.key === 'status'">
                <span class="font-mono text-xs font-bold"
                  :class="record.status < 400 ? 'text-green-500' : 'text-red-500'">
                  {{ record.status }}
                </span>
              </template>
              <template v-if="column.key === 'url'">
                <span class="text-xs font-medium text-macos-text truncate block max-w-md">{{ record.url }}</span>
              </template>
              <template v-if="column.key === 'timestamp'">
                <span class="text-[11px] text-macos-text-secondary font-medium">{{ record.timestamp }}</span>
              </template>
            </template>
          </a-table>
        </div>
        <!-- Footer Info -->
        <div
          class="h-8 border-t border-macos-border bg-black/5 dark:bg-white/5 flex items-center px-4 justify-between shrink-0">
          <span class="text-[10px] font-bold text-macos-text-secondary opacity-50 uppercase tracking-widest">
            已抓取 {{ trafficLogs.length }} 条实时审计记录
          </span>
          <span class="text-[10px] font-bold text-macos-text-secondary opacity-50 uppercase tracking-widest">
            全时段审计监控中
          </span>
        </div>
      </div>

      <!-- Detail Panel (Conditionally visible) -->
      <transition name="slide-panel">
        <div v-if="selectedRow"
          class="w-80 bg-white/60 dark:bg-white/10 border border-macos-border rounded-xl shadow-lg flex flex-col overflow-hidden">
          <div class="p-4 border-b border-macos-border flex items-center justify-between">
            <h4 class="text-sm font-bold text-macos-text">连接详细详情</h4>
            <button @click="selectedRow = null" class="p-1 hover:bg-black/5 dark:hover:bg-white/10 rounded">
              <ChevronRight class="w-4 h-4 text-macos-text-secondary" />
            </button>
          </div>

          <div class="flex-1 overflow-auto p-4 space-y-6">
            <div class="space-y-4">
              <div class="flex flex-col gap-1">
                <span class="text-[10px] font-bold uppercase text-macos-text-secondary opacity-50">远程目标地址</span>
                <div class="flex items-center gap-2 text-sm font-bold text-macos-text">
                  <Globe class="w-3.5 h-3.5 text-macos-accent" />
                  {{ selectedRow.dst_addr }}
                </div>
              </div>

              <div class="flex flex-col gap-1">
                <span class="text-[10px] font-bold uppercase text-macos-text-secondary opacity-50">本地发起地址</span>
                <div class="flex items-center gap-2 text-sm font-bold text-macos-text">
                  <ShieldCheck class="w-3.5 h-3.5 text-green-500" />
                  {{ selectedRow.src_addr }}
                </div>
              </div>

              <div class="flex flex-col gap-1">
                <span class="text-[10px] font-bold uppercase text-macos-text-secondary opacity-50">记录时间</span>
                <div class="flex items-center gap-2 text-sm font-bold text-macos-text">
                  <Clock class="w-3.5 h-3.5 text-blue-500" />
                  {{ selectedRow.timestamp }}
                </div>
              </div>
            </div>

            <div class="pt-4 border-t border-macos-border">
              <span class="text-[10px] font-bold uppercase text-macos-text-secondary opacity-50 block mb-2">传输指标</span>
              <div class="grid grid-cols-2 gap-3">
                <div class="bg-black/5 dark:bg-white/5 p-2 rounded-lg">
                  <div class="text-[10px] text-macos-text-secondary mb-0.5">数据量</div>
                  <div class="text-xs font-bold">{{ formatBytes(selectedRow.size) }}</div>
                </div>
                <div class="bg-black/5 dark:bg-white/5 p-2 rounded-lg">
                  <div class="text-[10px] text-macos-text-secondary mb-0.5">往返延迟</div>
                  <div class="text-xs font-bold">{{ Math.floor(Math.random() * 200) }}ms</div>
                </div>
              </div>
            </div>

            <div class="pt-4 border-t border-macos-border">
              <span class="text-[10px] font-bold uppercase text-macos-text-secondary opacity-50 block mb-2">完整访问
                URL</span>
              <div
                class="text-[11px] font-mono break-all bg-black/5 dark:bg-white/5 p-3 rounded-lg border border-macos-border/30">
                {{ selectedRow.url }}
              </div>
            </div>
          </div>

          <div class="p-4 bg-black/5 dark:bg-white/5 border-t border-macos-border">
            <button
              class="w-full py-2 bg-macos-accent text-white rounded-lg text-xs font-bold hover:bg-macos-accent-hover transition-colors">
              审查数据包详情
            </button>
          </div>
        </div>
      </transition>
    </div>
  </div>
</template>

<style scoped>
.macos-table :deep(.ant-table) {
  background: transparent !important;
}

.macos-table :deep(.ant-table-thead > tr > th) {
  background: transparent !important;
  color: var(--macos-text-secondary) !important;
  font-size: 10px !important;
  font-weight: 700 !important;
  text-transform: uppercase !important;
  letter-spacing: 0.05em !important;
  border-bottom: 1px solid var(--macos-border) !important;
  padding: 8px 12px !important;
}

.macos-table :deep(.ant-table-tbody > tr > td) {
  border-bottom: 1px solid var(--macos-border) !important;
  padding: 8px 12px !important;
  cursor: pointer;
}

.macos-table :deep(.ant-table-tbody > tr:hover > td) {
  background: rgba(var(--macos-accent-rgb, 0, 122, 255), 0.05) !important;
}

.macos-table :deep(.ant-table-placeholder) {
  background: transparent !important;
  border: none !important;
}

/* Transitions */
.slide-panel-enter-active,
.slide-panel-leave-active {
  transition: all 0.3s cubic-bezier(0.22, 1, 0.36, 1);
}

.slide-panel-enter-from,
.slide-panel-leave-to {
  opacity: 0;
  transform: translateX(20px);
  width: 0;
  margin-left: -1rem;
}
</style>
