<script setup lang="ts">
import { ref } from 'vue';
import {
  Monitor,
  ShieldAlert,
  Camera,
  CheckCircle2,
  TrendingUp,
  AlertTriangle
} from 'lucide-vue-next';

const systemStatus = ref({
  serviceRunning: true,
  auditCount: 128,
  riskCount: 2,
  lastSync: '10:23',
});

const stats = [
  { label: '今日审计流量', value: '128', unit: '条记录', icon: Monitor, color: 'text-blue-500', bg: 'bg-blue-500/10' },
  { label: '危险行为拦截', value: '2', unit: '次拦截', icon: ShieldAlert, color: 'text-red-500', bg: 'bg-red-500/10' },
  { label: '界面审计快照', value: '12', unit: '张图片', icon: Camera, color: 'text-purple-500', bg: 'bg-purple-500/10' },
  { label: '系统运行状态', value: '健康', unit: '', icon: CheckCircle2, color: 'text-green-500', bg: 'bg-green-500/10' },
];
</script>

<template>
  <div class="space-y-8 animate-in fade-in slide-in-from-bottom-2 duration-500">
    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <div v-for="stat in stats" :key="stat.label"
        class="bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl p-4 shadow-sm hover:shadow-md transition-all duration-300 group">
        <div class="flex items-center gap-3">
          <div :class="[stat.bg, stat.color]" class="p-2.5 rounded-lg transition-transform group-hover:scale-110">
            <component :is="stat.icon" class="w-5 h-5" />
          </div>
          <div class="flex flex-col">
            <span class="text-[10px] font-bold uppercase tracking-wider text-macos-text-secondary opacity-60">{{
              stat.label }}</span>
            <div class="flex items-baseline gap-1">
              <span class="text-xl font-bold text-macos-text tracking-tight">{{ stat.value }}</span>
              <span class="text-[10px] text-macos-text-secondary font-medium">{{ stat.unit }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Content Sections -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Traffic Chart Placeholder -->
      <div
        class="lg:col-span-2 bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl p-6 shadow-sm overflow-hidden flex flex-col">
        <div class="flex items-center justify-between mb-6">
          <div class="flex items-center gap-2">
            <TrendingUp class="w-4 h-4 text-macos-accent" />
            <h3 class="font-semibold text-macos-text">审计频率趋势</h3>
          </div>
          <select class="bg-black/5 dark:bg-white/5 border-none text-[10px] font-bold rounded px-2 py-1 outline-none">
            <option>最近 24 小时</option>
            <option>最近 7 天</option>
          </select>
        </div>

        <div class="flex-1 min-h-[240px] flex items-center justify-center relative">
          <!-- Mock Chart UI -->
          <div class="absolute inset-0 flex items-end justify-between px-2 opacity-20">
            <div v-for="i in 20" :key="i" :style="{ height: `${Math.random() * 80 + 20}%` }"
              class="w-2 bg-macos-accent rounded-t-sm"></div>
          </div>
          <div class="z-10 text-center">
            <p class="text-xs font-semibold text-macos-text-secondary mb-1">实时审计流已开启</p>
            <p class="text-[10px] text-macos-text-secondary opacity-50 italic">同步刷新本地审计事件中...</p>
          </div>
        </div>
      </div>

      <!-- Alerts / Recent Events -->
      <div class="bg-white/40 dark:bg-white/5 border border-macos-border rounded-xl p-6 shadow-sm flex flex-col">
        <div class="flex items-center gap-2 mb-6">
          <AlertTriangle class="w-4 h-4 text-red-500" />
          <h3 class="font-semibold text-macos-text">最近风险预警</h3>
        </div>

        <div class="space-y-4">
          <div v-for="i in 3" :key="i"
            class="flex gap-3 p-3 rounded-lg hover:bg-black/5 dark:hover:bg-white/5 transition-colors cursor-pointer group">
            <div class="w-1.5 h-1.5 rounded-full bg-red-500 mt-1.5 shrink-0 group-hover:scale-125 transition-transform">
            </div>
            <div class="flex flex-col gap-0.5">
              <span class="text-xs font-bold text-macos-text">检测到异常连接重定向</span>
              <span class="text-[10px] text-macos-text-secondary line-clamp-1">进程: curl (PID: 1452) -> 1.1.1.1:80</span>
              <span class="text-[9px] font-medium text-macos-text-secondary opacity-50 uppercase tracking-tighter">{{ i
                * 2 }} 分钟前</span>
            </div>
          </div>

          <button
            class="w-full py-2 mt-4 text-[10px] font-bold uppercase tracking-widest text-macos-accent hover:bg-macos-accent/10 rounded-md transition-colors border border-macos-accent/20">
            查看全部风险记录
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.animate-in {
  animation: fadeIn 0.6s cubic-bezier(0.22, 1, 0.36, 1);
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
