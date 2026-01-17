<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import {
  ShieldCheck,
  AlertTriangle,
  RefreshCcw,
  Search,
  FileCode,
  FileText,
  Trash2,
  Edit3
} from 'lucide-vue-next';

interface FileLog {
  id: string;
  timestamp: string;
  process: string;
  operation: string;
  file_path: string;
  result: 'allowed' | 'denied';
}

const loading = ref(false);
const fileLogs = ref<FileLog[]>([]);

const columns = [
  { title: '操作时间', dataIndex: 'timestamp', key: 'timestamp', width: 120 },
  { title: '来源进程', dataIndex: 'process', key: 'process', width: 130 },
  { title: '操作行为', dataIndex: 'operation', key: 'operation', width: 90 },
  { title: '受保护路径', dataIndex: 'file_path', key: 'file_path', ellipsis: true },
  { title: '处置状态', dataIndex: 'result', key: 'result', width: 90 },
];

const loadFileLogs = async () => {
  loading.value = true;
  try {
    // Mock Data
    fileLogs.value = Array.from({ length: 12 }, (_, i) => ({
      id: `file-${i}`,
      timestamp: new Date(Date.now() - i * 180000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      process: ['Finder', 'Terminal', 'VSCode', 'clash'][i % 4],
      operation: ['OPEN', 'WRITE', 'DELETE', 'RENAME'][i % 4],
      file_path: `/Library/Application Support/MacMonitor/data/${['config', 'logs', 'cache'][i % 3]}_${i}.db`,
      result: i % 5 === 0 ? 'denied' : 'allowed',
    }));
  } catch (err) {
    message.error('无法加载文件监控日志');
  } finally {
    loading.value = false;
  }
};

onMounted(loadFileLogs);

const getOpIcon = (op: string) => {
  switch (op) {
    case 'WRITE': return Edit3;
    case 'DELETE': return Trash2;
    case 'OPEN': return FileText;
    default: return FileCode;
  }
};
</script>

<template>
  <div class="h-full flex flex-col space-y-6 animate-in fade-in duration-500">
    <!-- Header banner -->
    <div class="bg-indigo-500/10 border border-indigo-500/20 rounded-xl p-4 flex items-start gap-4 mx-1">
      <div class="bg-indigo-500/20 p-2 rounded-lg text-indigo-600 dark:text-indigo-400">
        <ShieldCheck class="w-5 h-5" />
      </div>
      <div class="flex-1">
        <h4 class="text-sm font-bold text-macos-text">文件系统完整性保护已开启</h4>
        <p class="text-xs text-macos-text-secondary opacity-80 leading-relaxed">
          正在实时监控核心系统路径及应用敏感数据。任何未经授权的对此类保护文件的修改或删除操作都将被拦截记录。
        </p>
      </div>
      <button @click="loadFileLogs"
        class="p-2 hover:bg-black/5 dark:hover:bg-white/5 rounded-lg transition-colors shrink-0"
        :class="{ 'animate-spin': loading }">
        <RefreshCcw class="w-4 h-4 text-macos-text-secondary" />
      </button>
    </div>

    <!-- Filter Bar -->
    <div class="flex items-center gap-2 mx-1">
      <div class="relative flex-1">
        <Search class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-macos-text-secondary opacity-50" />
        <input type="text" placeholder="搜索文件路径或进程名称..."
          class="h-9 w-full bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl pl-9 pr-3 text-xs outline-none focus:ring-1 focus:ring-macos-accent transition-all" />
      </div>
    </div>

    <!-- Log Table -->
    <div
      class="flex-1 overflow-hidden bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl shadow-sm flex flex-col mx-1">
      <div class="flex-1 overflow-auto">
        <a-table :columns="columns" :data-source="fileLogs" :loading="loading" :pagination="false" row-key="id"
          size="small" class="macos-table">
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'operation'">
              <div class="flex items-center gap-1.5 font-bold text-[10px] tracking-tight">
                <component :is="getOpIcon(record.operation)" class="w-3 h-3 text-macos-text-secondary" />
                <span class="uppercase opacity-70">{{ record.operation }}</span>
              </div>
            </template>
            <template v-if="column.key === 'result'">
              <div class="flex items-center gap-1.5">
                <div class="w-1.5 h-1.5 rounded-full"
                  :class="record.result === 'denied' ? 'bg-red-500' : 'bg-green-500'"></div>
                <span class="text-[10px] font-bold uppercase tracking-tight"
                  :class="record.result === 'denied' ? 'text-red-500' : 'text-green-600'">
                  {{ record.result === 'denied' ? '已拦截' : '已放行' }}
                </span>
              </div>
            </template>
            <template v-if="column.key === 'file_path'">
              <span class="text-xs font-mono opacity-80 break-all select-all">{{ record.file_path }}</span>
            </template>
            <template v-if="column.key === 'process'">
              <span class="text-xs font-semibold text-macos-text">{{ record.process }}</span>
            </template>
            <template v-if="column.key === 'timestamp'">
              <span class="text-[11px] text-macos-text-secondary font-medium">{{ record.timestamp }}</span>
            </template>
          </template>
        </a-table>
      </div>
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
}

.macos-table :deep(.ant-table-tbody > tr > td) {
  border-bottom: 1px solid var(--macos-border) !important;
}

.animate-in {
  animation: fadeIn 0.5s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(5px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
