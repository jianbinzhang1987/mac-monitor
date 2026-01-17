<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import { invoke } from '@tauri-apps/api/core';
import { convertFileSrc } from '@tauri-apps/api/core';

interface Screenshot {
  id: number;
  capture_time: string;
  app_name: string;
  risk_level: number;
  ocr_text: string | null;
  image_path: string;
  redaction_labels: string | null;
  display_path?: string;
}

const loading = ref(false);
const screenshots = ref<Screenshot[]>([]);
const privacyMode = ref(true);

const togglePrivacyMode = async (checked: boolean) => {
  try {
    const res = await invoke<string>('set_redaction_status', { enabled: checked });
    message.success(checked ? '隐私保护模式已开启' : '隐私保护模式已关闭');
    privacyMode.value = checked;
  } catch (err) {
    message.error('切换隐私模式失败: ' + err);
    privacyMode.value = !checked; // 恢复状态
  }
};

const loadScreenshots = async () => {
  loading.value = true;
  try {
    const res = await invoke<Screenshot[]>('get_screenshot_logs');
    console.log('Fetched screenshots:', res);
    screenshots.value = res.map(s => ({
      ...s,
      // 转换本地文件路径为 WebView 可访问的 URL
      display_path: convertFileSrc(s.image_path)
    }));
  } catch (err) {
    message.error('加载截图记录失败: ' + err);
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

const getLabelColor = (label: string) => {
  const colors: Record<string, string> = {
    'PatternMatch': 'red',
    'KeywordInline': 'orange',
    'KeywordNext': 'gold',
    'NLP-Name': 'purple'
  };
  return colors[label] || 'blue';
};

const getLabelDesc = (label: string) => {
  const descs: Record<string, string> = {
    'PatternMatch': '正则模式匹配 (身份证/手机/卡号/邮箱/IP/车牌)',
    'KeywordInline': '标签内联敏感词 (如 用户名: xxx)',
    'KeywordNext': '标签关联敏感信息 (如 密码后面紧跟的内容)',
    'NLP-Name': 'NLP 语义识别出的姓名'
  };
  return descs[label] || '检测到敏感信息';
};
</script>

<template>
  <div class="screenshot-page">
    <div class="page-header">
      <h3 class="page-title">界面截图记录</h3>
      <div class="actions">
        <div class="privacy-toggle">
          <span class="label">隐私保护模式</span>
          <a-switch :checked="privacyMode" @change="togglePrivacyMode" />
        </div>
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
          <img :src="shot.display_path" :alt="shot.app_name" v-if="shot.display_path" />
          <div v-else class="no-image">无预览</div>

          <div v-if="shot.risk_level > 0" class="sensitive-badge">
            <a-tag color="error">敏感内容</a-tag>
          </div>
        </div>
        <div class="screenshot-info">
          <div class="info-row">
            <span class="label">应用：</span>
            <a-tag color="blue">{{ shot.app_name }}</a-tag>
          </div>
          <div class="info-row" v-if="shot.redaction_labels">
            <span class="label">识别原因：</span>
            <div class="labels-container">
              <a-tooltip v-for="label in shot.redaction_labels.split(',')" :key="label">
                <template #title>{{ getLabelDesc(label) }}</template>
                <a-tag :color="getLabelColor(label)">
                  {{ label }}
                </a-tag>
              </a-tooltip>
            </div>
          </div>
          <div class="info-row">
            <span class="label">时间：</span>
            <span class="value">{{ shot.capture_time }}</span>
          </div>
          <div class="info-row">
            <span class="label">OCR：</span>
            <span class="value ocr-text">{{ shot.ocr_text || '未提取到文本' }}</span>
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
  align-items: center;
  gap: 20px;
}

.privacy-toggle {
  display: flex;
  align-items: center;
  gap: 8px;
}

.labels-container {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
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
  display: flex;
  align-items: center;
  justify-content: center;
}

.screenshot-preview img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.no-image {
  color: #64748b;
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
  flex-wrap: wrap;
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
