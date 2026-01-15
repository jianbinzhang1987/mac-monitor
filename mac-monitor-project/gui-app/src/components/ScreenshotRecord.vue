<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';

interface Screenshot {
  id: string;
  timestamp: string;
  app_name: string;
  is_sensitive: boolean;
  ocr_text: string;
  thumbnail_path: string;
}

const loading = ref(false);
const screenshots = ref<Screenshot[]>([]);

const loadScreenshots = async () => {
  loading.value = true;
  try {
    // Mock 数据
    screenshots.value = Array.from({ length: 8 }, (_, i) => ({
      id: `shot-${i}`,
      timestamp: new Date(Date.now() - i * 300000).toLocaleString('zh-CN'),
      app_name: ['WeChat', 'QQ', 'Safari', 'Chrome'][i % 4],
      is_sensitive: i % 3 === 0,
      ocr_text: i % 3 === 0 ? '检测到敏感关键词：密码、账号' : '无敏感内容',
      thumbnail_path: `https://via.placeholder.com/300x200?text=Screenshot+${i}`,
    }));
  } catch (err) {
    message.error('加载截图记录失败');
    console.error(err);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadScreenshots();
});

const handleViewDetail = (screenshot: Screenshot) => {
  message.info(`查看截图详情: ${screenshot.id}`);
};
</script>

<template>
  <div class="screenshot-page">
    <div class="page-header">
      <h3 class="page-title">界面截图记录</h3>
      <div class="actions">
        <a-button type="primary" @click="loadScreenshots" :loading="loading">
          刷新
        </a-button>
      </div>
    </div>

    <a-alert
      message="智能截图监控运行中"
      description="针对社交应用（微信、QQ）和浏览器进行智能截图，敏感内容已脱敏处理"
      type="warning"
      show-icon
      style="margin-bottom: 16px"
    />

    <div class="screenshot-grid" v-if="!loading">
      <div v-for="shot in screenshots" :key="shot.id" class="screenshot-card card-glass">
        <div class="screenshot-preview">
          <img :src="shot.thumbnail_path" :alt="shot.app_name" />
          <div v-if="shot.is_sensitive" class="sensitive-badge">
            <a-tag color="error">敏感内容</a-tag>
          </div>
        </div>
        <div class="screenshot-info">
          <div class="info-row">
            <span class="label">应用：</span>
            <a-tag color="blue">{{ shot.app_name }}</a-tag>
          </div>
          <div class="info-row">
            <span class="label">时间：</span>
            <span class="value">{{ shot.timestamp }}</span>
          </div>
          <div class="info-row">
            <span class="label">OCR：</span>
            <span class="value ocr-text">{{ shot.ocr_text }}</span>
          </div>
          <a-button type="link" size="small" @click="handleViewDetail(shot)">
            查看详情
          </a-button>
        </div>
      </div>
    </div>

    <div v-else class="loading-container">
      <a-spin size="large" />
    </div>
  </div>
</template>

<style scoped>
.screenshot-page {
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

.screenshot-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.screenshot-card {
  padding: 0;
  overflow: hidden;
  transition: transform 0.3s, box-shadow 0.3s;
}

.screenshot-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(59, 130, 246, 0.2);
}

.screenshot-preview {
  position: relative;
  width: 100%;
  height: 200px;
  overflow: hidden;
  background: rgba(0, 0, 0, 0.3);
}

.screenshot-preview img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.sensitive-badge {
  position: absolute;
  top: 10px;
  right: 10px;
}

.screenshot-info {
  padding: 16px;
}

.info-row {
  margin-bottom: 8px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.label {
  color: #94a3b8;
  font-size: 13px;
}

.value {
  color: #e2e8f0;
  font-size: 13px;
}

.ocr-text {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-glass {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 12px;
}

.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 400px;
}

:deep(.ant-alert) {
  background: rgba(245, 158, 11, 0.1) !important;
  border: 1px solid rgba(245, 158, 11, 0.2) !important;
}
</style>
