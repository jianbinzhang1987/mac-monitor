<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import { invoke } from '@tauri-apps/api/core';

interface ClipboardLog {
  id: number;
  app_name: string;
  bundle_id: string;
  op_time: string;
  content: string;
  content_type: string;
  risk_level: number;
  cpe_id: string;
  host_id: string;
  mac: string;
  ip: string;
}

const loading = ref(false);
const clipboardLogs = ref<ClipboardLog[]>([]);

const columns = [
  { title: '时间', dataIndex: 'op_time', key: 'op_time', width: 180 },
  { title: '应用名称', dataIndex: 'app_name', key: 'app_name', width: 150 },
  { title: 'Bundle ID', dataIndex: 'bundle_id', key: 'bundle_id', ellipsis: true },
  { title: '内容类型', dataIndex: 'content_type', key: 'content_type', width: 120 },
  { title: '内容预览', dataIndex: 'content', key: 'content', ellipsis: true },
  { title: '风险等级', dataIndex: 'risk_level', key: 'risk_level', width: 100 },
];

const loadClipboardLogs = async () => {
  loading.value = true;
  try {
    const res = await invoke('get_clipboard_logs') as ClipboardLog[];
    clipboardLogs.value = res;
  } catch (err) {
    message.error('加载剪贴板日志失败');
    console.error(err);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadClipboardLogs();
});
</script>

<template>
  <div class="clipboard-audit-page">
    <div class="page-header">
      <h3 class="page-title">浏览器剪贴板审计</h3>
      <div class="actions">
        <a-button type="primary" @click="loadClipboardLogs" :loading="loading">
          刷新
        </a-button>
      </div>
    </div>

    <a-alert
      message="剪贴板监控已激活"
      description="仅监控白名单浏览器的剪贴板写入行为 (Chrome, Safari, Firefox, Edge, Arc 等)"
      type="info"
      show-icon
      style="margin-bottom: 16px"
    />

    <div class="table-container card-glass">
      <a-table
        :columns="columns"
        :data-source="clipboardLogs"
        :loading="loading"
        :pagination="{ pageSize: 10 }"
        row-key="id"
        size="small"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'content_type'">
            <a-tag :color="record.content_type === 'text/plain' ? 'blue' : (record.content_type === 'text/url' ? 'green' : 'orange')">
              {{ record.content_type }}
            </a-tag>
          </template>
          <template v-if="column.key === 'risk_level'">
            <a-tag :color="record.risk_level > 1 ? 'error' : 'warning'">
              {{ record.risk_level > 1 ? '高风险' : '信息' }}
            </a-tag>
          </template>
        </template>
      </a-table>
    </div>
  </div>
</template>

<style scoped>
.clipboard-audit-page {
  padding: 24px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  color: #f8fafc;
  margin: 0;
}

.actions {
  display: flex;
  gap: 12px;
}

.table-container {
  padding: 16px;
}

.card-glass {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 12px;
}

:deep(.ant-table) {
  background: transparent !important;
  color: #e2e8f0 !important;
}

:deep(.ant-table-thead > tr > th) {
  background: rgba(255, 255, 255, 0.05) !important;
  color: #94a3b8 !important;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1) !important;
}

:deep(.ant-table-tbody > tr > td) {
  border-bottom: 1px solid rgba(255, 255, 255, 0.05) !important;
  color: #cbd5e1 !important;
}

:deep(.ant-table-tbody > tr:hover > td) {
  background: rgba(59, 130, 246, 0.05) !important;
}

:deep(.ant-alert) {
  background: rgba(59, 130, 246, 0.1) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
  color: #e2e8f0 !important;
}
</style>
