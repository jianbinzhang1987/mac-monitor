<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';

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
  { title: '时间', dataIndex: 'timestamp', key: 'timestamp', width: 180 },
  { title: '进程', dataIndex: 'process', key: 'process', width: 150 },
  { title: '操作', dataIndex: 'operation', key: 'operation', width: 100 },
  { title: '文件路径', dataIndex: 'file_path', key: 'file_path', ellipsis: true },
  { title: '结果', dataIndex: 'result', key: 'result', width: 100 },
];

const loadFileLogs = async () => {
  loading.value = true;
  try {
    // Mock 数据
    fileLogs.value = Array.from({ length: 12 }, (_, i) => ({
      id: `file-${i}`,
      timestamp: new Date(Date.now() - i * 180000).toLocaleString('zh-CN'),
      process: ['Finder', 'Terminal', 'VSCode', 'clash'][i % 4],
      operation: ['OPEN', 'WRITE', 'DELETE', 'RENAME'][i % 4],
      file_path: `/Library/Application Support/MacMonitor/config${i}.db`,
      result: i % 5 === 0 ? 'denied' : 'allowed',
    }));
  } catch (err) {
    message.error('加载文件日志失败');
    console.error(err);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadFileLogs();
});
</script>

<template>
  <div class="file-security-page">
    <div class="page-header">
      <h3 class="page-title">文件安全监控</h3>
      <div class="actions">
        <a-button type="primary" @click="loadFileLogs" :loading="loading">
          刷新
        </a-button>
      </div>
    </div>

    <a-alert
      message="文件保护已激活"
      description="受保护路径：/Library/Application Support/MacMonitor, /var/log/macmonitor"
      type="info"
      show-icon
      style="margin-bottom: 16px"
    />

    <div class="table-container card-glass">
      <a-table
        :columns="columns"
        :data-source="fileLogs"
        :loading="loading"
        :pagination="{ pageSize: 10 }"
        row-key="id"
        size="small"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'operation'">
            <a-tag :color="['blue', 'green', 'red', 'orange'][['OPEN', 'WRITE', 'DELETE', 'RENAME'].indexOf(record.operation)]">
              {{ record.operation }}
            </a-tag>
          </template>
          <template v-if="column.key === 'result'">
            <a-tag :color="record.result === 'denied' ? 'error' : 'success'">
              {{ record.result === 'denied' ? '已拦截' : '已放行' }}
            </a-tag>
          </template>
        </template>
      </a-table>
    </div>
  </div>
</template>

<style scoped>
.file-security-page {
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
}
</style>
