<script setup lang="ts">
import { ref } from 'vue'
import Login from './components/Login.vue'
import Dashboard from './components/Dashboard.vue'

const isLoggedIn = ref(false)

const handleLoginSuccess = () => {
  isLoggedIn.value = true
}
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
