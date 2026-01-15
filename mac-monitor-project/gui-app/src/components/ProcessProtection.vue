<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';

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
  { title: '时间', dataIndex: 'timestamp', key: 'timestamp', width: 180 },
  { title: '进程名称', dataIndex: 'process_name', key: 'process_name', width: 180 },
  { title: '操作类型', dataIndex: 'action', key: 'action', width: 120 },
  { title: '目标对象', dataIndex: 'target', key: 'target', ellipsis: true },
  { title: '处理结果', dataIndex: 'result', key: 'result', width: 100 },
  { title: '原因', dataIndex: 'reason', key: 'reason', ellipsis: true },
];

const loadProcessLogs = async () => {
  loading.value = true;
  try {
    // Mock 数据
    processLogs.value = Array.from({ length: 15 }, (_, i) => ({
      id: `proc-${i}`,
      timestamp: new Date(Date.now() - i * 120000).toLocaleString('zh-CN'),
      process_name: ['clash-verge', 'v2ray', 'shadowsocks', 'surge', 'Terminal'][i % 5],
      action: ['SIGNAL', 'KILL', 'TERMINATE'][i % 3],
      target: 'AuditService',
      result: i % 4 === 0 ? 'blocked' : 'allowed',
      reason: i % 4 === 0 ? '未授权的进程终止尝试' : '正常系统操作',
    }));
  } catch (err) {
    message.error('加载进程日志失败');
    console.error(err);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadProcessLogs();
});

const getResultTag = (result: string) => {
  return result === 'blocked' 
    ? { color: 'error', text: '已拦截' } 
    : { color: 'success', text: '已放行' };
};
</script>

<template>
  <div class="process-protection-page">
    <div class="page-header">
      <h3 class="page-title">进程防护日志</h3>
      <div class="actions">
        <a-button type="primary" @click="loadProcessLogs" :loading="loading">
          刷新
        </a-button>
      </div>
    </div>

    <a-alert
      message="进程防护已启用"
      description="系统正在实时监控并拦截未授权的进程终止行为，保护关键服务不被恶意终止。"
      type="success"
      show-icon
      closable
      style="margin-bottom: 16px"
    />

    <div class="table-container card-glass">
      <a-table
        :columns="columns"
        :data-source="processLogs"
        :loading="loading"
        :pagination="{ pageSize: 10 }"
        row-key="id"
        size="small"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'process_name'">
            <a-tag color="purple">{{ record.process_name }}</a-tag>
          </template>
          <template v-if="column.key === 'action'">
            <a-tag color="orange">{{ record.action }}</a-tag>
          </template>
          <template v-if="column.key === 'result'">
            <a-tag :color="getResultTag(record.result).color">
              {{ getResultTag(record.result).text }}
            </a-tag>
          </template>
        </template>
      </a-table>
    </div>
  </div>
</template>

<style scoped>
.process-protection-page {
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
  background: rgba(16, 185, 129, 0.1) !important;
  border: 1px solid rgba(16, 185, 129, 0.2) !important;
}
</style>
