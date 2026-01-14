<script setup lang="ts">
import { ref, reactive } from 'vue'
import { message } from 'ant-design-vue'
import { invoke } from '@tauri-apps/api/core'

const activeKey = ref('login')

const loginForm = reactive({
  pin: '',
})

const registerForm = reactive({
  serverIp: '',
  serverPort: '',
  cpeId: '',
  pin: '',
})

const emit = defineEmits(['loginSuccess'])

const handleLogin = async () => {
  if (!loginForm.pin) {
    message.error('请输入 PIN 码')
    return
  }

  try {
    // For development/demo purposes, bypass actual backend invoke if needed or keep it
    // const res = await invoke('login', { payload: { pin: loginForm.pin } })
    // message.success(res as string)

    // Simulating success for now if backend isn't fully ready or strictly for UI flow
    message.success('登录成功')
    emit('loginSuccess')
  } catch (err) {
    message.error('登录失败: ' + err)
  }
}

const handleRegister = async () => {
  if (!registerForm.serverIp || !registerForm.serverPort || !registerForm.cpeId || !registerForm.pin) {
    message.error('请填写完整的注册信息')
    return
  }

  try {
    const res = await invoke('register', {
      payload: {
        server_ip: registerForm.serverIp,
        server_port: registerForm.serverPort,
        cpe_id: registerForm.cpeId,
        pin: registerForm.pin
      }
    })
    message.success(res as string)
  } catch (err) {
    message.error('注册失败: ' + err)
  }
}
</script>

<template>
  <div class="login-wrapper">
    <a-card style="width: 400px" title="MAC 终端审计系统">
      <a-tabs v-model:activeKey="activeKey">
        <a-tab-pane key="login" tab="用户登录">
          <a-form layout="vertical">
            <a-form-item label="PIN 码">
              <a-input v-model:value="loginForm.pin" placeholder="请输入您的 PIN 码" />
            </a-form-item>
            <a-form-item>
              <a-button type="primary" block @click="handleLogin">登录</a-button>
            </a-form-item>
          </a-form>
        </a-tab-pane>
        <a-tab-pane key="register" tab="设备注册">
          <a-form layout="vertical">
            <a-form-item label="服务器 IP">
              <a-input v-model:value="registerForm.serverIp" placeholder="例如: 10.1.1.1" />
            </a-form-item>
            <a-form-item label="服务器端口">
              <a-input v-model:value="registerForm.serverPort" placeholder="例如: 8080" />
            </a-form-item>
            <a-form-item label="CPE ID">
              <a-input v-model:value="registerForm.cpeId" placeholder="请输入 CPE 设备 ID" />
            </a-form-item>
            <a-form-item label="PIN 码">
              <a-input v-model:value="registerForm.pin" placeholder="请输入 PIN 码" />
            </a-form-item>
            <a-form-item>
              <a-button type="primary" block @click="handleRegister">注册</a-button>
            </a-form-item>
          </a-form>
        </a-tab-pane>
      </a-tabs>
    </a-card>
  </div>
</template>

<style scoped>
.login-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background-color: #f0f2f5;
}
</style>
