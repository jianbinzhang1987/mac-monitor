<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { message } from 'ant-design-vue';

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
const selectedRowKeys = ref<string[]>([]);

const columns = [
  { title: '时间', dataIndex: 'timestamp', key: 'timestamp', width: 180 },
  { title: '协议', dataIndex: 'protocol', key: 'protocol', width: 80 },
  { title: '源地址', dataIndex: 'src_addr', key: 'src_addr', width: 150 },
  { title: '目标地址', dataIndex: 'dst_addr', key: 'dst_addr', width: 150 },
  { title: 'URL', dataIndex: 'url', key: 'url', ellipsis: true },
  { title: '方法', dataIndex: 'method', key: 'method', width: 80 },
  { title: '状态', dataIndex: 'status', key: 'status', width: 80 },
  { title: '大小', dataIndex: 'size', key: 'size', width: 100 },
];

const loadTrafficLogs = async () => {
  loading.value = true;
  try {
    // TODO: 调用 Tauri command 获取真实数据
    // const logs = await invoke('get_traffic_logs');
    // trafficLogs.value = logs;
    
    // Mock 数据
    trafficLogs.value = Array.from({ length: 20 }, (_, i) => ({
      id: `log-${i}`,
      timestamp: new Date(Date.now() - i * 60000).toLocaleString('zh-CN'),
      protocol: ['HTTPS', 'HTTP', 'WS'][i % 3],
      src_addr: `192.168.1.${100 + i}`,
      dst_addr: `203.208.${40 + (i % 10)}.${100 + i}`,
      url: `https://example${i}.com/api/v1/data`,
      method: ['GET', 'POST', 'PUT'][i % 3],
      status: [200, 201, 304, 404][i % 4],
      size: Math.floor(Math.random() * 50000) + 1000,
    }));
  } catch (err) {
    message.error('加载流量日志失败');
    console.error(err);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadTrafficLogs();
});

const handleExport = () => {
  message.info('导出功能开发中...');
};

const formatBytes = (bytes: number) => {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
};
</script>

<template>
  <div class="traffic-audit-page">
    <div class="page-header">
      <h3 class="page-title">网络流量审计</h3>
      <div class="actions">
        <a-button type="primary" @click="loadTrafficLogs" :loading="loading">
          刷新
        </a-button>
        <a-button @click="handleExport">导出记录</a-button>
      </div>
    </div>

    <div class="filter-bar card-glass">
      <a-space>
        <a-select v-model:value="protocol" placeholder="协议类型" style="width: 120px" allowClear>
          <a-select-option value="HTTP">HTTP</a-select-option>
          <a-select-option value="HTTPS">HTTPS</a-select-option>
          <a-select-option value="WS">WebSocket</a-select-option>
        </a-select>
        <a-input placeholder="搜索 URL 或 IP" style="width: 200px" />
        <a-range-picker />
      </a-space>
    </div>

    <div class="table-container card-glass">
      <a-table
        :columns="columns"
        :data-source="trafficLogs"
        :loading="loading"
        :pagination="{ pageSize: 10 }"
        :row-selection="{ selectedRowKeys, onChange: (keys) => selectedRowKeys = keys }"
        row-key="id"
        size="small"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'protocol'">
            <a-tag :color="record.protocol === 'HTTPS' ? 'green' : 'blue'">
              {{ record.protocol }}
            </a-tag>
          </template>
          <template v-if="column.key === 'status'">
            <a-tag :color="record.status < 300 ? 'success' : 'warning'">
              {{ record.status }}
            </a-tag>
          </template>
          <template v-if="column.key === 'size'">
            {{ formatBytes(record.size) }}
          </template>
        </template>
      </a-table>
    </div>
  </div>
</template>

<style scoped>
.traffic-audit-page {
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

.filter-bar {
  padding: 16px;
  margin-bottom: 16px;
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
</style>
