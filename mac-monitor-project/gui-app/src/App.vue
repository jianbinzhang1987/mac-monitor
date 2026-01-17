<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Login from './components/Login.vue'
import Dashboard from './components/Dashboard.vue'
import MacLayout from './layouts/MacLayout.vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const isLoggedIn = ref(false)

const handleLoginSuccess = async () => {
  isLoggedIn.value = true
  try {
    // Use static method to get the main window specifically
    const { Window } = await import('@tauri-apps/api/window')
    const mainWin = await Window.getByLabel('main')
    if (mainWin) {
      await mainWin.maximize()
    }
  } catch (err) {
    console.error('Failed to maximize window:', err)
  }
}

onMounted(async () => {
  try {
    // Globally sync device info on app launch
    const info = await invoke('get_system_device_info')
    console.log('Global Device Info Sync:', info)
    await invoke('set_device_info', { payload: info })
  } catch (err) {
    console.error('Failed to sync global device info:', err)
  }
})
</script>

<template>
  <MacLayout v-if="isLoggedIn">
    <Dashboard />
  </MacLayout>

  <div v-else class="h-full w-full bg-macos-bg flex items-center justify-center" data-tauri-drag-region>
    <!-- Simple Drag Region for Login if not in Layout -->
    <div class="absolute top-0 left-0 w-full h-8 z-50" data-tauri-drag-region></div>
    <Login @loginSuccess="handleLoginSuccess" />
  </div>
</template>

<style>
/* Global styles moved to style.css and tailwind layers */
</style>
