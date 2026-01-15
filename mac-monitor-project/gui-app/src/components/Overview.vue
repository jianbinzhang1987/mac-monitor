<script setup lang="ts">
import { ref } from 'vue';

const systemStatus = ref({
  serviceRunning: true,
  auditCount: 128,
  riskCount: 2,
  lastSync: '10:23',
});
</script>

<template>
  <div class="overview-page">
    <div class="page-header">
      <h2 class="page-title">系统实时概览</h2>
      <div class="date-time">{{ systemStatus.lastSync }} 最后同步</div>
    </div>

    <a-row :gutter="24" class="stat-cards">
      <a-col :span="6">
        <div class="card-glass stat-card">
          <div class="stat-icon-wrap blue"><desktop-outlined /></div>
          <div class="stat-content">
            <div class="stat-label">今日流量审计</div>
            <div class="stat-value">{{ systemStatus.auditCount }} <span class="unit">条</span></div>
          </div>
        </div>
      </a-col>
      <a-col :span="6">
        <div class="card-glass stat-card">
          <div class="stat-icon-wrap red"><safety-certificate-outlined /></div>
          <div class="stat-content">
            <div class="stat-label">威胁风险拦截</div>
            <div class="stat-value">{{ systemStatus.riskCount }} <span class="unit">次</span></div>
          </div>
        </div>
      </a-col>
      <a-col :span="6">
        <div class="card-glass stat-card">
          <div class="stat-icon-wrap green"><camera-outlined /></div>
          <div class="stat-content">
            <div class="stat-label">敏感截图留存</div>
            <div class="stat-value">12 <span class="unit">张</span></div>
          </div>
        </div>
      </a-col>
      <a-col :span="6">
        <div class="card-glass stat-card">
          <div class="stat-icon-wrap orange"><sync-outlined /></div>
          <div class="stat-content">
            <div class="stat-label">后台服务状态</div>
            <div class="stat-value">健康</div>
          </div>
        </div>
      </a-col>
    </a-row>

    <div class="main-card card-glass mt-24">
      <a-tabs default-active-key="1">
        <a-tab-pane key="1" tab="近期流量动态">
          <div class="chart-placeholder">
            <div class="mock-line"></div>
            <p>实时流量曲线图加载中...</p>
          </div>
        </a-tab-pane>
        <a-tab-pane key="2" tab="终端行为预警">
          <a-empty description="当前环境安全，暂无异常行为" />
        </a-tab-pane>
      </a-tabs>
    </div>
  </div>
</template>

<script lang="ts">
import {
  DesktopOutlined,
  SafetyCertificateOutlined,
  CameraOutlined,
  SyncOutlined,
} from '@ant-design/icons-vue';

export default {
  components: {
    DesktopOutlined,
    SafetyCertificateOutlined,
    CameraOutlined,
    SyncOutlined,
  },
};
</script>

<style scoped>
.overview-page {
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

.stat-icon-wrap.blue { background: rgba(59, 130, 246, 0.1); color: #3b82f6; }
.stat-icon-wrap.red { background: rgba(244, 63, 94, 0.1); color: #f43f5e; }
.stat-icon-wrap.green { background: rgba(16, 185, 129, 0.1); color: #10b981; }
.stat-icon-wrap.orange { background: rgba(245, 158, 11, 0.1); color: #f59e0b; }

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

.mt-24 { margin-top: 24px; }

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
</style>
