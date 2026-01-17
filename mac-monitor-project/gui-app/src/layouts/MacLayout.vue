<script setup lang="ts">
import TitleBar from '../components/macos/TitleBar.vue';
import Sidebar from '../components/macos/Sidebar.vue';
</script>

<template>
  <div class="flex flex-col w-full h-full bg-macos-bg backdrop-blur-3xl overflow-hidden rounded-xl border border-macos-border shadow-macos-window">
    <!-- Top Drag Region / TitleBar -->
    <div class="absolute top-0 left-0 w-full z-50">
      <TitleBar />
    </div>

    <!-- Main Content Area -->
    <div class="flex flex-1 pt-10 h-full overflow-hidden relative">
      <!-- Sidebar -->
      <Sidebar />

      <!-- Main View (Content) -->
      <main class="flex-1 h-full bg-white/50 dark:bg-black/20 backdrop-blur-md overflow-hidden flex flex-col relative">
        <div class="flex-1 overflow-auto p-6 scrollbar-hide">
          <slot />
        </div>
      </main>
    </div>
  </div>
</template>
