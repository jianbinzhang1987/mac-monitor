<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Login from './components/Login.vue'
import Dashboard from './components/Dashboard.vue'

const isLoggedIn = ref(false)

const handleLoginSuccess = () => {
  isLoggedIn.value = true
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
  <Dashboard v-if="isLoggedIn" />
  <Login v-else @loginSuccess="handleLoginSuccess" />
</template>

<style>
body {
  margin: 0;
  padding: 0;
}
</style>
