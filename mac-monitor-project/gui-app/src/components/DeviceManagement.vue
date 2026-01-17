<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
    Laptop,
    Fingerprint,
    Network,
    Globe,
    Server,
    Clock,
    RefreshCw,
    HardDrive,
    Cpu,
    ShieldCheck,
    Activity,
    Box
} from 'lucide-vue-next';

interface DeviceInfo {
    pin_number: string;
    ip: string;
    mac: string;
    cpe_id: string;
    host_id: string;
}

const deviceInfo = ref<DeviceInfo | null>(null);
const loading = ref(true);

const fetchDeviceInfo = async () => {
    loading.value = true;
    try {
        const info = await invoke<DeviceInfo>('get_system_device_info');
        deviceInfo.value = info;
    } catch (err) {
        console.error('Failed to fetch device info:', err);
    } finally {
        loading.value = false;
    }
};

onMounted(fetchDeviceInfo);

const infoGroups = [
    {
        title: '硬件身份与凭据',
        icon: Fingerprint,
        color: 'text-purple-500',
        bg: 'bg-purple-500/10',
        items: [
            { label: 'CPE 唯一识别码', key: 'cpe_id' },
            { label: '主机实例 ID', key: 'host_id' },
            { label: '授权 PIN 码', key: 'pin_number', sensitive: true }
        ]
    },
    {
        title: '网络适配器状态',
        icon: Network,
        color: 'text-blue-500',
        bg: 'bg-blue-500/10',
        items: [
            { label: '局域网 IPv4 地址', key: 'ip' },
            { label: '物理 MAC 地址', key: 'mac' }
        ]
    }
];
</script>

<template>
    <div class="h-full flex flex-col space-y-10 animate-in fade-in duration-700 max-w-5xl mx-auto px-4">
        <!-- Header: Apple Style System Banner -->
        <header class="flex flex-col items-center text-center space-y-6 pt-6">
            <div class="relative group">
                <div
                    class="absolute inset-0 bg-macos-accent blur-3xl opacity-10 group-hover:opacity-20 transition-opacity duration-700">
                </div>
                <div
                    class="w-28 h-28 rounded-[2rem] bg-white dark:bg-black/40 shadow-2xl flex items-center justify-center border border-macos-border relative overflow-hidden group-hover:scale-105 transition-transform duration-500">
                    <div class="absolute inset-0 bg-gradient-to-br from-transparent via-macos-accent/5 to-transparent">
                    </div>
                    <Laptop class="w-14 h-14 text-macos-text opacity-90 relative z-10" />
                    <!-- Animated Ring -->
                    <div
                        class="absolute inset-0 border-[3px] border-macos-accent/20 rounded-[2rem] animate-[ping_3s_infinite] opacity-30">
                    </div>
                </div>
            </div>

            <div class="space-y-1">
                <h2 class="text-3xl font-black tracking-tighter text-macos-text">终端工作站概览</h2>
                <div class="flex items-center justify-center gap-3">
                    <span
                        class="px-2 py-0.5 rounded-md bg-macos-text/5 text-[9px] font-black uppercase tracking-[0.2em] text-macos-text-secondary">Release
                        2026</span>
                    <div class="w-1 h-1 rounded-full bg-macos-text-secondary opacity-30"></div>
                    <span class="text-[10px] font-bold text-macos-text-secondary opacity-60">System Version 2.1.4
                        (24A335)</span>
                </div>
            </div>
        </header>

        <!-- Detailed Info Cards -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <section v-for="group in infoGroups" :key="group.title"
                class="bg-white/40 dark:bg-white/5 border border-macos-border rounded-3xl p-6 shadow-sm hover:shadow-xl transition-all duration-500 group/card">
                <div class="flex items-center gap-3 mb-6">
                    <div :class="[group.bg, group.color]" class="p-2 rounded-xl">
                        <component :is="group.icon" class="w-5 h-5" />
                    </div>
                    <h3 class="text-xs font-black uppercase tracking-[0.15em] text-macos-text opacity-70">{{ group.title
                        }}</h3>
                </div>

                <div class="space-y-4">
                    <div v-for="item in group.items" :key="item.key" class="flex flex-col gap-1.5 group/item">
                        <span
                            class="text-[10px] text-macos-text-secondary font-black uppercase tracking-widest opacity-40 ml-1">{{
                            item.label }}</span>
                        <div
                            class="flex items-center justify-between bg-black/5 dark:bg-white/5 p-3 rounded-2xl border border-transparent group-hover/item:border-macos-accent/20 group-hover/item:bg-macos-accent/5 transition-all outline-none">
                            <code class="text-xs font-mono font-bold text-macos-text tracking-tight">
                {{ deviceInfo?.[item.key as keyof DeviceInfo] || '--' }}
              </code>
                            <Box
                                class="w-3.5 h-3.5 text-macos-text-secondary opacity-10 group-hover/item:opacity-30 transition-opacity" />
                        </div>
                    </div>
                </div>
            </section>

            <!-- Connection Logic Control Room -->
            <section
                class="bg-white/40 dark:bg-white/5 border border-macos-border rounded-3xl p-8 shadow-sm md:col-span-2 relative overflow-hidden">
                <div
                    class="absolute top-0 right-0 w-64 h-64 bg-green-500/5 blur-[80px] rounded-full translate-x-1/2 -translate-y-1/2 pointer-events-none">
                </div>

                <div class="flex items-center justify-between mb-8">
                    <div class="flex items-center gap-3">
                        <div class="p-2.5 rounded-2xl bg-green-500/10 text-green-600">
                            <Server class="w-6 h-6" />
                        </div>
                        <div>
                            <h3 class="text-sm font-black uppercase tracking-[0.1em] text-macos-text">实时连接状态</h3>
                            <p class="text-[10px] text-macos-text-secondary font-bold opacity-60">中心端下发审计策略已就绪</p>
                        </div>
                    </div>

                    <div
                        class="flex items-center gap-2.5 px-4 py-1.5 rounded-2xl bg-green-500/10 border border-green-500/20 shadow-sm">
                        <span
                            class="w-2 h-2 rounded-full bg-green-500 animate-pulse shadow-[0_0_10px_rgba(34,197,94,0.6)]"></span>
                        <span
                            class="text-[11px] font-black text-green-600 dark:text-green-400 uppercase tracking-widest">Connected</span>
                    </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-3 gap-8">
                    <div
                        class="p-4 rounded-2xl bg-black/5 dark:bg-white/5 border border-macos-border/30 hover:shadow-md transition-shadow">
                        <div
                            class="flex items-center gap-2 text-[10px] text-macos-text-secondary font-black uppercase tracking-widest opacity-40 mb-2">
                            <Globe class="w-3.5 h-3.5" /> 审计服务器
                        </div>
                        <span class="text-base font-black text-macos-text">10.211.55.2 : 8080</span>
                    </div>
                    <div
                        class="p-4 rounded-2xl bg-black/5 dark:bg-white/5 border border-macos-border/30 hover:shadow-md transition-shadow">
                        <div
                            class="flex items-center gap-2 text-[10px] text-macos-text-secondary font-black uppercase tracking-widest opacity-40 mb-2">
                            <Clock class="w-3.5 h-3.5" /> 已同步时长
                        </div>
                        <span class="text-base font-black text-macos-text">1h 24m</span>
                    </div>
                    <div
                        class="p-4 rounded-2xl bg-black/5 dark:bg-white/5 border border-macos-border/30 hover:shadow-md transition-shadow">
                        <div
                            class="flex items-center gap-2 text-[10px] text-macos-text-secondary font-black uppercase tracking-widest opacity-40 mb-2">
                            <ShieldCheck class="w-3.5 h-3.5" /> 拦截器版本
                        </div>
                        <span class="text-base font-black text-macos-text">v2.1.0-Core</span>
                    </div>
                </div>

                <div
                    class="mt-8 pt-8 border-t border-macos-border flex flex-col sm:flex-row justify-between items-center gap-4">
                    <div
                        class="flex items-center gap-2 text-[10px] font-bold text-macos-text-secondary opacity-40 group cursor-default">
                        <Activity class="w-3.5 h-3.5 group-hover:animate-bounce" />
                        数据包完整性校验: 100% Passed
                    </div>
                    <div class="flex gap-4">
                        <button @click="fetchDeviceInfo"
                            class="flex items-center gap-2 px-5 h-11 bg-black/5 dark:bg-white/10 hover:bg-black/10 transition-all rounded-2xl text-xs font-black text-macos-text-secondary">
                            <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': loading }" />
                            拉取最新配置
                        </button>
                        <button class="flex items-center gap-2 px-6 h-11 btn-vibrant rounded-2xl text-xs">
                            <RefreshCw class="w-4 h-4" />
                            立即全局同步
                        </button>
                    </div>
                </div>
            </section>
        </div>

        <!-- System Specs Footer -->
        <footer
            class="flex flex-col sm:flex-row items-center justify-between opacity-30 mt-auto pt-10 pb-4 border-t border-macos-border">
            <div
                class="flex items-center gap-6 text-[9px] font-black uppercase tracking-[0.25em] text-macos-text-secondary">
                <div class="flex items-center gap-2 hover:opacity-100 transition-opacity">
                    <Cpu class="w-4 h-4" /> Neural Engine Active
                </div>
                <div class="flex items-center gap-2 hover:opacity-100 transition-opacity">
                    <HardDrive class="w-4 h-4" /> Root Partition Verified
                </div>
            </div>
            <div class="mt-4 sm:mt-0 px-3 py-1 bg-macos-text-secondary/10 rounded-lg">
                <span class="text-[9px] font-mono font-black text-macos-text-secondary opacity-50 select-all">UUID:
                    F81D4FAE-7DEC-11D0-A765-00A0C91E6BF6</span>
            </div>
        </footer>
    </div>
</template>

<style scoped>
.animate-in {
    animation: slideIn 0.8s cubic-bezier(0.22, 1, 0.36, 1);
}

@keyframes slideIn {
    from {
        opacity: 0;
        transform: translateY(20px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
}
</style>
