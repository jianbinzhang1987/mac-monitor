<script setup lang="ts">
// TitleBar.vue
// Handles the top draggable area and window controls spacing
</script>

<template>
  <div data-tauri-drag-region class="h-10 w-full flex items-center justify-center relative bg-transparent shrink-0">
    <!-- Traffic lights placeholder (handled by OS, but we need space) -->
    <div class="absolute left-4 top-0 h-full flex items-center pointer-events-none">
      <!-- Native buttons sit here -->
    </div>

    <!-- Title -->
    <span class="text-sm font-medium text-macos-text-secondary opacity-80 pointer-events-none select-none">
      互联网终端审计系统
    </span>
  </div>
</template>
