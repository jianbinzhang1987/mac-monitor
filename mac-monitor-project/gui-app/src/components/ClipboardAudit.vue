<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import { invoke } from '@tauri-apps/api/core';
import {
  Clipboard,
  Search,
  RefreshCcw,
  Link2,
  Type,
  AlertCircle,
  ShieldCheck,
  Globe
} from 'lucide-vue-next';

interface ClipboardLog {
  id: number;
  app_name: string;
  bundle_id: string;
  op_time: string;
  content: string;
  content_type: string;
  risk_level: number;
}

const loading = ref(false);
const clipboardLogs = ref<ClipboardLog[]>([]);

const columns = [
  { title: '记录时间', dataIndex: 'op_time', key: 'op_time', width: 120 },
  { title: '来源应用', dataIndex: 'app_name', key: 'app_name', width: 140 },
  { title: '数据类型', dataIndex: 'content_type', key: 'content_type', width: 100 },
  { title: '剪贴板预览', dataIndex: 'content', key: 'content', ellipsis: true },
  { title: '风险等级', dataIndex: 'risk_level', key: 'risk_level', width: 80 },
];

const loadClipboardLogs = async () => {
  loading.value = true;
  try {
    const res = await invoke('get_clipboard_logs') as ClipboardLog[];
    clipboardLogs.value = res;
  } catch (err) {
    message.error('剪贴板监控流连接中断');
  } finally {
    loading.value = false;
  }
};

onMounted(loadClipboardLogs);
</script>

<template>
  <div class="h-full flex flex-col space-y-6 animate-in fade-in duration-500">
    <!-- Header banner -->
    <div class="bg-amber-500/10 border border-amber-500/20 rounded-xl p-4 flex items-start gap-4 mx-1">
      <div class="bg-amber-500/20 p-2 rounded-lg text-amber-600 dark:text-amber-400">
        <Clipboard class="w-5 h-5" />
      </div>
      <div class="flex-1">
        <h4 class="text-sm font-bold text-macos-text">浏览器剪贴板行为审计已激活</h4>
        <p class="text-xs text-macos-text-secondary opacity-80 leading-relaxed">
          正在监控终端侧浏览器级别的剪贴板写入行为。重点审计 Safari, Chrome, Arc 等主流办公浏览器中涉及敏感数据的跨应用拷贝。
        </p>
      </div>
      <button @click="loadClipboardLogs"
        class="p-2 hover:bg-black/5 dark:hover:bg-white/5 rounded-lg transition-colors shrink-0"
        :class="{ 'animate-spin': loading }">
        <RefreshCcw class="w-4 h-4 text-macos-text-secondary" />
      </button>
    </div>

    <!-- Quick Stats & Filters -->
    <div class="flex items-center gap-4 mx-1">
      <div class="relative flex-1">
        <Search class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-macos-text-secondary opacity-50" />
        <input type="text" placeholder="过滤关键字、URL 或 敏感词..."
          class="h-10 w-full bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl pl-9 pr-3 text-xs outline-none focus:ring-1 focus:ring-macos-accent transition-all" />
      </div>
    </div>

    <!-- Log Table -->
    <div
      class="flex-1 overflow-hidden bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl shadow-sm flex flex-col mx-1">
      <div class="flex-1 overflow-auto">
        <a-table :columns="columns" :data-source="clipboardLogs" :loading="loading" :pagination="false" row-key="id"
          size="small" class="macos-table">
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'app_name'">
              <div class="flex items-center gap-2">
                <div class="w-5 h-5 rounded bg-black/5 dark:bg-white/10 flex items-center justify-center">
                  <Globe v-if="record.app_name.includes('Safari') || record.app_name.includes('Chrome')"
                    class="w-3 h-3 text-macos-accent" />
                  <Type v-else class="w-3 h-3 text-macos-text-secondary" />
                </div>
                <span class="text-xs font-semibold text-macos-text">{{ record.app_name }}</span>
              </div>
            </template>

            <template v-if="column.key === 'content_type'">
              <div class="flex items-center gap-1.5 opacity-70">
                <component :is="record.content_type.includes('url') ? Link2 : Type" class="w-3 h-3" />
                <span class="text-[10px] font-bold uppercase tracking-tight">{{ record.content_type.split('/')[1] ||
                  '文本' }}</span>
              </div>
            </template>

            <template v-if="column.key === 'risk_level'">
              <div class="flex items-center gap-1.5">
                <component :is="record.risk_level > 1 ? AlertCircle : ShieldCheck" class="w-3.5 h-3.5"
                  :class="record.risk_level > 1 ? 'text-red-500' : 'text-green-500'" />
                <span class="text-[10px] font-bold uppercase tracking-tight"
                  :class="record.risk_level > 1 ? 'text-red-600' : 'text-green-600'">
                  {{ record.risk_level > 1 ? '高风险' : '安全' }}
                </span>
              </div>
            </template>

            <template v-if="column.key === 'content'">
              <span
                class="text-xs font-medium text-macos-text opacity-90 select-all underline decoration-macos-accent/20 underline-offset-2">{{
                  record.content }}</span>
            </template>

            <template v-if="column.key === 'op_time'">
              <span class="text-[11px] text-macos-text-secondary font-medium">{{ record.op_time }}</span>
            </template>
          </template>
        </a-table>
      </div>

      <!-- Table Footer -->
      <div class="px-4 h-9 border-t border-macos-border bg-black/5 dark:bg-white/5 flex items-center shrink-0">
        <span
          class="text-[9px] font-bold uppercase tracking-[0.2em] text-macos-text-secondary opacity-40">已开启隐私数据脱敏技术</span>
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
  padding-top: 12px !important;
}

.macos-table :deep(.ant-table-tbody > tr > td) {
  border-bottom: 1px solid var(--macos-border) !important;
  padding: 10px 12px !important;
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
