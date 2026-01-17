<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import {
  ShieldCheck,
  ShieldAlert,
  RefreshCcw,
  Info,
  Cpu,
  Activity,
  Zap
} from 'lucide-vue-next';

interface ProcessLog {
  id: string;
  timestamp: string;
  process_name: string;
  action: string;
  target: string;
  result: 'allowed' | 'blocked';
  reason: string;
}

const loading = ref(false);
const processLogs = ref<ProcessLog[]>([]);

const columns = [
  { title: '触发时间', dataIndex: 'timestamp', key: 'timestamp', width: 120 },
  { title: '来源进程', dataIndex: 'process_name', key: 'process_name', width: 140 },
  { title: '操作行为', dataIndex: 'action', key: 'action', width: 100 },
  { title: '拦截目标', dataIndex: 'target', key: 'target', ellipsis: true },
  { title: '处置结果', dataIndex: 'result', key: 'result', width: 90 },
];

const loadProcessLogs = async () => {
  loading.value = true;
  try {
    // Mock Data
    processLogs.value = Array.from({ length: 15 }, (_, i) => ({
      id: `proc-${i}`,
      timestamp: new Date(Date.now() - i * 120000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      process_name: ['clash-verge', 'v2ray', 'shadowsocks', 'surge', 'Terminal'][i % 5],
      action: ['SIGNAL', 'KILL', 'TERMINATE'][i % 3],
      target: 'AuditService',
      result: i % 4 === 0 ? 'blocked' : 'allowed',
      reason: i % 4 === 0 ? 'Unauthorized termination attempt' : 'Normal system operation',
    }));
  } catch (err) {
    message.error('加载防护日志失败');
  } finally {
    loading.value = false;
  }
};

onMounted(loadProcessLogs);
</script>

<template>
  <div class="h-full flex flex-col space-y-6 animate-in fade-in duration-500">
    <!-- Header banner -->
    <div class="bg-blue-500/10 border border-blue-500/20 rounded-xl p-4 flex items-start gap-4 mx-1">
      <div class="bg-blue-500/20 p-2 rounded-lg text-blue-600 dark:text-blue-400">
        <ShieldCheck class="w-5 h-5" />
      </div>
      <div class="flex-1">
        <h4 class="text-sm font-bold text-macos-text">核心组件进程防护已开启</h4>
        <p class="text-xs text-macos-text-secondary opacity-80 leading-relaxed">
          系统正在实时监控关键服务的退出与终止尝试。任何未经授权的非法进程终止信号都将被自动拦截并审计。
        </p>
      </div>
      <button @click="loadProcessLogs"
        class="p-2 hover:bg-black/5 dark:hover:bg-white/5 rounded-lg transition-colors shrink-0"
        :class="{ 'animate-spin': loading }">
        <RefreshCcw class="w-4 h-4 text-macos-text-secondary" />
      </button>
    </div>

    <!-- Quick Stats -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 px-1">
      <div
        class="bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl p-4 shadow-sm flex items-center gap-3">
        <div class="p-2 rounded-lg bg-purple-500/10 text-purple-600">
          <Cpu class="w-4 h-4" />
        </div>
        <div>
          <div class="text-[10px] font-bold uppercase tracking-wider text-macos-text-secondary opacity-60">重点监控中
          </div>
          <div class="text-sm font-bold">128 个存活进程</div>
        </div>
      </div>
      <div
        class="bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl p-4 shadow-sm flex items-center gap-3">
        <div class="p-2 rounded-lg bg-orange-500/10 text-orange-600">
          <Activity class="w-4 h-4" />
        </div>
        <div>
          <div class="text-[10px] font-bold uppercase tracking-wider text-macos-text-secondary opacity-60">今日捕获信号
          </div>
          <div class="text-sm font-bold">45 次系统事件</div>
        </div>
      </div>
      <div
        class="bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl p-4 shadow-sm flex items-center gap-3">
        <div class="p-2 rounded-lg bg-red-500/10 text-red-600">
          <Zap class="w-4 h-4" />
        </div>
        <div>
          <div class="text-[10px] font-bold uppercase tracking-wider text-macos-text-secondary opacity-60">已拦截风险
          </div>
          <div class="text-sm font-bold">3 次异常终止尝试</div>
        </div>
      </div>
    </div>

    <!-- Log Table -->
    <div
      class="flex-1 overflow-hidden bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl shadow-sm flex flex-col mx-1">
      <div class="px-4 py-3 border-b border-macos-border flex items-center justify-between">
        <h4 class="text-xs font-bold uppercase tracking-widest text-macos-text-secondary opacity-50">底层内核事件流</h4>
        <div class="flex items-center gap-2">
          <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
          <span class="text-[10px] font-bold text-macos-text-secondary uppercase">监控运行中</span>
        </div>
      </div>

      <div class="flex-1 overflow-auto">
        <a-table :columns="columns" :data-source="processLogs" :loading="loading" :pagination="false" row-key="id"
          size="small" class="macos-table">
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'process_name'">
              <div class="flex items-center gap-2">
                <div class="w-1.5 h-1.5 rounded-full bg-purple-500"></div>
                <span class="text-xs font-semibold text-macos-text">{{ record.process_name }}</span>
              </div>
            </template>
            <template v-if="column.key === 'action'">
              <span
                class="text-[10px] font-mono font-bold text-orange-600 dark:text-orange-400 px-1.5 py-0.5 rounded bg-orange-500/10">
                {{ record.action }}
              </span>
            </template>
            <template v-if="column.key === 'result'">
              <div class="flex items-center gap-1.5">
                <component :is="record.result === 'allowed' ? ShieldCheck : ShieldAlert" class="w-3.5 h-3.5"
                  :class="record.result === 'allowed' ? 'text-green-500' : 'text-red-500'" />
                <span class="text-[10px] font-bold uppercase tracking-tight"
                  :class="record.result === 'allowed' ? 'text-green-600' : 'text-red-600'">
                  {{ record.result === 'allowed' ? '已放行' : '已拦截' }}
                </span>
              </div>
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
