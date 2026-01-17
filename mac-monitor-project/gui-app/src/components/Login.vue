<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { invoke } from '@tauri-apps/api/core'
import { KeyRound, ShieldCheck, Server, Network, Cpu, Lock, Fingerprint } from 'lucide-vue-next';

type DeviceInfo = {
  pin_number: string
  ip: string
  mac: string
  cpe_id: string
  host_id: string
}

const activeKey = ref('login')

const loginForm = reactive({
  pin: '',
})

const registerForm = reactive({
  serverIp: '',
  serverPort: '',
  cpeId: '',
  pin: '',
  ip: '',
  mac: '',
})

const emit = defineEmits(['loginSuccess'])

const handleLogin = async () => {
  if (!loginForm.pin) {
    message.error('请输入 PIN 码')
    return
  }

  try {
    const res = await invoke('login', { payload: { pin: loginForm.pin } })
    message.success('身份验证已通过')
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
        pin: registerForm.pin,
        ip: registerForm.ip,
        mac: registerForm.mac
      }
    })
    message.success('终端注册成功')
    activeKey.value = 'login'
  } catch (err) {
    message.error('注册失败: ' + err)
  }
}

onMounted(async () => {
  try {
    const info = await invoke('get_system_device_info') as DeviceInfo
    registerForm.cpeId = info.cpe_id
    registerForm.pin = info.pin_number
    Object.assign(registerForm, {
      ip: info.ip,
      mac: info.mac
    })

    await invoke('set_device_info', { payload: info })
  } catch (err) {
    console.error('Failed to get system device info:', err)
  }
})
</script>

<template>
  <div class="fixed inset-0 overflow-hidden flex items-center justify-center p-6 bg-mesh-gradient"
    data-tauri-drag-region>
    <!-- Background Floating Accents -->
    <div
      class="absolute top-[-10%] left-[-10%] w-[50%] h-[50%] bg-macos-accent/10 blur-[150px] rounded-full animate-pulse pointer-events-none">
    </div>
    <div
      class="absolute bottom-[-10%] right-[-10%] w-[50%] h-[50%] bg-purple-600/10 blur-[150px] rounded-full animate-pulse duration-[7000ms] pointer-events-none">
    </div>

    <!-- Premium Window Frame -->
    <main
      class="w-[440px] thick-glass rounded-[28px] flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-700 select-none">

      <!-- Logo & Header -->
      <section class="pt-10 pb-6 px-10 flex flex-col items-center gap-5 text-center">
        <div class="relative group">
          <div
            class="absolute inset-0 bg-macos-accent blur-xl opacity-20 group-hover:opacity-40 transition-opacity duration-500">
          </div>
          <div
            class="w-16 h-16 rounded-2xl bg-gradient-to-br from-[#30cfd0] to-[#330867] dark:from-macos-accent dark:to-blue-700 shadow-2xl flex items-center justify-center relative transform transition-transform group-hover:scale-105 duration-500">
            <ShieldCheck class="w-10 h-10 text-white" />
          </div>
        </div>
        <div>
          <h1 class="text-2xl font-black tracking-tight text-macos-text">互联网终端审计系统</h1>
          <p class="text-[10px] font-bold text-macos-text-secondary opacity-50 uppercase tracking-[0.3em] mt-2">Terminal
            Monitoring Node</p>
        </div>
      </section>

      <!-- Segmented Control (Tabs) -->
      <div class="px-10 flex justify-center">
        <div class="w-full h-11 bg-black/5 dark:bg-white/10 rounded-2xl p-1 flex relative">
          <button @click="activeKey = 'login'"
            class="flex-1 rounded-xl text-[13px] font-bold transition-all duration-500 z-10"
            :class="activeKey === 'login' ? 'text-macos-text' : 'text-macos-text-secondary hover:text-macos-text opacity-60 hover:opacity-100'">
            授权登录
          </button>
          <button @click="activeKey = 'register'"
            class="flex-1 rounded-xl text-[13px] font-bold transition-all duration-500 z-10"
            :class="activeKey === 'register' ? 'text-macos-text' : 'text-macos-text-secondary hover:text-macos-text opacity-60 hover:opacity-100'">
            设备注册
          </button>
          <!-- Slider -->
          <div
            class="absolute top-1 left-1 h-9 w-[calc(50%-4px)] bg-white dark:bg-white/10 rounded-xl shadow-[0_2px_8px_rgba(0,0,0,0.1)] transition-transform duration-500 ease-[cubic-bezier(0.22,1,0.36,1)]"
            :class="activeKey === 'register' ? 'translate-x-[calc(100%+0px)]' : 'translate-x-0'"></div>
        </div>
      </div>

      <!-- Content Area -->
      <div class="flex-1 p-10 min-h-[320px]">
        <transition name="view-switch" mode="out-in">
          <!-- Login View -->
          <div v-if="activeKey === 'login'" :key="'login'" class="flex flex-col gap-8">
            <div class="space-y-3">
              <div class="flex items-center justify-between px-1">
                <label
                  class="text-[10px] font-black text-macos-text-secondary uppercase tracking-widest opacity-40">终端授权码</label>
                <Fingerprint class="w-3 h-3 text-macos-accent opacity-20" />
              </div>
              <div class="relative group">
                <Lock
                  class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-macos-text-secondary opacity-30 group-focus-within:text-macos-accent group-focus-within:opacity-100 transition-all" />
                <input type="password" v-model="loginForm.pin" placeholder="请输入 6-12 位授权 PIN 码"
                  class="w-full h-12 bg-white/50 dark:bg-black/20 border border-macos-border rounded-xl pl-12 pr-4 text-sm outline-none focus:ring-4 focus:ring-macos-accent/15 focus:border-macos-accent/50 transition-all font-medium placeholder:opacity-30"
                  @keyup.enter="handleLogin" />
              </div>
            </div>

            <button @click="handleLogin" class="w-full h-12 btn-vibrant rounded-2xl text-sm">
              验证身份并进入系统
            </button>
          </div>

          <!-- Register View -->
          <div v-else :key="'register'" class="flex flex-col gap-5 max-h-[380px] overflow-y-auto pr-1">
            <div class="grid grid-cols-2 gap-4">
              <div class="space-y-2 col-span-2">
                <label
                  class="text-[10px] font-black text-macos-text-secondary uppercase tracking-widest opacity-40 px-1">管理节点服务器</label>
                <div class="relative">
                  <Server
                    class="absolute left-3.5 top-1/2 -translate-y-1/2 w-4 h-4 text-macos-text-secondary opacity-30" />
                  <input v-model="registerForm.serverIp" placeholder="服务器 IP 地址"
                    class="w-full h-11 bg-white/50 dark:bg-black/20 border border-macos-border rounded-xl pl-11 pr-4 text-xs font-medium outline-none focus:ring-2 focus:ring-macos-accent/20 transition-all" />
                </div>
              </div>

              <div class="space-y-2">
                <label
                  class="text-[10px] font-black text-macos-text-secondary uppercase tracking-widest opacity-40 px-1">端口</label>
                <input v-model="registerForm.serverPort" placeholder="8080"
                  class="w-full h-11 bg-white/50 dark:bg-black/20 border border-macos-border rounded-xl px-4 text-xs font-medium outline-none focus:ring-2 focus:ring-macos-accent/20 transition-all" />
              </div>

              <div class="space-y-2">
                <label
                  class="text-[10px] font-black text-macos-text-secondary uppercase tracking-widest opacity-40 px-1">CPE
                  ID</label>
                <input v-model="registerForm.cpeId" placeholder="设备唯一编号"
                  class="w-full h-11 bg-white/50 dark:bg-black/20 border border-macos-border rounded-xl px-4 text-xs font-medium outline-none focus:ring-2 focus:ring-macos-accent/20 transition-all" />
              </div>

              <div class="space-y-2 col-span-2">
                <label
                  class="text-[10px] font-black text-macos-text-secondary uppercase tracking-widest opacity-40 px-1">绑定授权码</label>
                <div class="relative">
                  <KeyRound
                    class="absolute left-3.5 top-1/2 -translate-y-1/2 w-4 h-4 text-macos-text-secondary opacity-30" />
                  <input type="password" v-model="registerForm.pin" placeholder="输入节点授权密钥"
                    class="w-full h-11 bg-white/50 dark:bg-black/20 border border-macos-border rounded-xl pl-11 pr-4 text-xs font-medium outline-none focus:ring-2 focus:ring-macos-accent/20 transition-all" />
                </div>
              </div>
            </div>

            <button @click="handleRegister" class="w-full h-11 btn-vibrant rounded-xl text-xs font-black shadow-lg">
              完成终端设备注册
            </button>
          </div>
        </transition>
      </div>

      <!-- Legal & Encryption Info -->
      <footer
        class="h-12 border-t border-macos-border bg-black/5 dark:bg-white/5 flex items-center px-10 justify-between shrink-0">
        <div class="flex items-center gap-2 opacity-30 group cursor-help">
          <Network class="w-3.5 h-3.5 text-macos-text-secondary group-hover:text-macos-accent transition-colors" />
          <span class="text-[9px] font-black uppercase tracking-[0.2em] text-macos-text-secondary">SSL-P2P
            Encrypted</span>
        </div>
        <span
          class="text-[9px] font-black text-macos-text-secondary opacity-20 uppercase tracking-widest">v2.1.0-Release</span>
      </footer>
    </main>
  </div>
</template>

<style scoped>
.view-switch-enter-active,
.view-switch-leave-active {
  transition: all 0.4s cubic-bezier(0.22, 1, 0.36, 1);
}

.view-switch-enter-from {
  opacity: 0;
  transform: scale(0.97) translateY(8px);
}

.view-switch-leave-to {
  opacity: 0;
  transform: scale(1.03) translateY(-8px);
}

/* Hide scrollbar but keep functionality */
.overflow-y-auto::-webkit-scrollbar {
  width: 0px;
}
</style>
