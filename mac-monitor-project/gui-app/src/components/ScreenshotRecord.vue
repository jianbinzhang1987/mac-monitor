<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import {
  Camera,
  ShieldCheck,
  ShieldAlert,
  RefreshCcw,
  Eye,
  EyeOff,
  Clock,
  ExternalLink,
  Search
} from 'lucide-vue-next';

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
    await invoke('set_redaction_status', { enabled: checked });
    message.success(checked ? '隐私自动脱敏已开启' : '隐私自动脱敏已关闭');
    privacyMode.value = checked;
  } catch (err) {
    message.error('同步隐私状态失败');
  }
};

const loadScreenshots = async () => {
  loading.value = true;
  try {
    const res = await invoke<Screenshot[]>('get_screenshot_logs');
    screenshots.value = res.map(s => ({
      ...s,
      display_path: convertFileSrc(s.image_path)
    }));
  } catch (err) {
    message.error('获取截图审计日志失败');
  } finally {
    loading.value = false;
  }
};

onMounted(loadScreenshots);

const getRiskColor = (level: number) => {
  return level > 0 ? 'bg-red-500 text-white' : 'bg-green-500 text-white';
};
</script>

<template>
  <div class="h-full flex flex-col space-y-6 animate-in fade-in duration-500">
    <!-- Toolbar -->
    <div class="flex items-center justify-between px-1 shrink-0">
      <div class="flex items-center gap-3">
        <div
          class="flex items-center gap-2 bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl px-4 h-10 shadow-sm">
          <span class="text-xs font-bold text-macos-text opacity-70">隐私内容自动遮罩</span>
          <a-switch :checked="privacyMode" @change="togglePrivacyMode" size="small" />
          <component :is="privacyMode ? EyeOff : Eye" class="w-3.5 h-3.5 text-macos-accent ml-1" />
        </div>
      </div>

      <div class="flex items-center gap-2">
        <button @click="loadScreenshots"
          class="p-2 hover:bg-black/5 dark:hover:bg-white/10 rounded-lg transition-colors shrink-0"
          :class="{ 'animate-spin': loading }">
          <RefreshCcw class="w-4 h-4 text-macos-text-secondary" />
        </button>
        <div class="relative">
          <Search class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-macos-text-secondary opacity-50" />
          <input type="text" placeholder="搜索 OCR 关键词..."
            class="h-10 w-48 bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl pl-9 pr-3 text-xs outline-none focus:ring-1 focus:ring-macos-accent transition-all" />
        </div>
      </div>
    </div>

    <!-- Captures Grid -->
    <div v-if="!loading" class="flex-1 overflow-auto px-1">
      <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-6">
        <div v-for="shot in screenshots" :key="shot.id"
          class="bg-white/40 dark:bg-white/5 border border-macos-border rounded-2xl overflow-hidden shadow-sm hover:shadow-xl transition-all duration-300 group flex flex-col">
          <!-- Preview Frame -->
          <div
            class="aspect-video bg-black/5 dark:bg-black/20 relative group-hover:scale-[1.02] transition-transform duration-500 overflow-hidden">
            <img v-if="shot.display_path" :src="shot.display_path"
              class="w-full h-full object-cover opacity-90 group-hover:opacity-100 transition-opacity" />
            <div v-else class="w-full h-full flex items-center justify-center">
              <Camera class="w-8 h-8 text-macos-text-secondary opacity-20" />
            </div>

            <!-- Risk Badge -->
            <div v-if="shot.risk_level > 0"
              class="absolute top-3 right-3 px-2 py-1 rounded-md bg-red-500/90 text-white text-[9px] font-bold uppercase tracking-wider shadow-lg backdrop-blur-md">
              检出敏感内容
            </div>
          </div>

          <!-- Meta Info -->
          <div class="p-4 flex flex-col gap-3">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <div class="w-2 h-2 rounded-full bg-macos-accent shadow-[0_0_8px_rgba(0,122,255,0.5)]"></div>
                <span class="text-xs font-bold text-macos-text">{{ shot.app_name }}</span>
              </div>
              <div class="flex items-center gap-1.5 text-macos-text-secondary opacity-60">
                <Clock class="w-3 h-3" />
                <span class="text-[10px] font-bold">{{ shot.capture_time }}</span>
              </div>
            </div>

            <div class="space-y-1.5">
              <p class="text-[11px] font-medium text-macos-text-secondary line-clamp-2 leading-relaxed h-8">
                {{ shot.ocr_text || '未从此帧提取到有效文本内容' }}
              </p>

              <div class="flex flex-wrap gap-1.5 pt-1">
                <span v-for="label in shot.redaction_labels?.split(',')" :key="label"
                  class="text-[9px] font-bold px-1.5 py-0.5 rounded-md bg-macos-text-secondary/10 text-macos-text-secondary border border-macos-text-secondary/10">
                  {{ label }}
                </span>
              </div>
            </div>

            <button
              class="mt-2 w-full flex items-center justify-center gap-2 py-2 rounded-xl bg-black/5 dark:bg-white/5 hover:bg-macos-accent hover:text-white dark:hover:bg-macos-accent text-[11px] font-bold transition-all duration-300">
              <ExternalLink class="w-3 h-3" />
              查看审计详情
            </button>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="screenshots.length === 0" class="h-64 flex flex-col items-center justify-center opacity-30">
        <Camera class="w-12 h-12 mb-4" />
        <p class="text-sm font-bold uppercase tracking-widest">暂无屏幕审计快照</p>
      </div>
    </div>

    <div v-else class="flex-1 flex items-center justify-center">
      <RefreshCcw class="w-8 h-8 text-macos-accent animate-spin opacity-50" />
    </div>
  </div>
</template>

<style scoped>
.animate-in {
  animation: fadeIn 0.6s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
