<script setup lang="ts">
import { LayoutDashboard, Activity, Shield, FileText, Camera, Clipboard, RefreshCw } from 'lucide-vue-next';
import { useNavigation } from '../../composables/useNavigation';

const { activeView, navigateTo } = useNavigation();

const navItems = [
    { id: 'overview', label: '系统概览', icon: LayoutDashboard },
    { id: 'traffic', label: '网络审计', icon: Activity },
    { id: 'process', label: '进程防护', icon: Shield },
    { id: 'file', label: '文件安全', icon: FileText },
    { id: 'screen', label: '界面快照', icon: Camera },
    { id: 'clipboard', label: '剪贴审计', icon: Clipboard },
    { id: 'sync', label: '终端管理', icon: RefreshCw },
];
</script>

<template>
    <aside
        class="w-60 h-full flex flex-col bg-macos-sidebar backdrop-blur-3xl border-r border-macos-border pt-4 px-3 pb-4 shrink-0 transition-colors duration-300">
        <!-- Clean Header/Logo Area -->
        <div class="px-3 mb-6 flex items-center gap-2 opacity-80">
            <div
                class="w-6 h-6 rounded-md bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-[10px] font-bold text-white shadow-sm">
                M</div>
            <span class="text-sm font-semibold tracking-tight text-macos-text">终端审计系统</span>
        </div>

        <!-- Navigation List -->
        <nav class="flex flex-col gap-0.5">
            <div class="px-3 py-1 mb-1 text-[10px] font-semibold text-macos-text-secondary uppercase opacity-60">
                实时审计监控
            </div>

            <button v-for="item in navItems" :key="item.id" @click="navigateTo(item.id)"
                class="group flex items-center gap-2.5 px-3 py-1.5 rounded-md text-sm font-medium transition-all duration-200"
                :class="[
                    activeView === item.id
                        ? 'bg-macos-accent text-white shadow-sm'
                        : 'text-macos-text hover:bg-black/5 dark:hover:bg-white/10 active:opacity-70'
                ]">
                <component :is="item.icon" class="w-4 h-4 transition-colors"
                    :class="[activeView === item.id ? 'text-white' : 'text-macos-text-secondary group-hover:text-macos-text']" />
                {{ item.label }}
            </button>
        </nav>
    </aside>
</template>
