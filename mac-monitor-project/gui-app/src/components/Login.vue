<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { invoke } from '@tauri-apps/api/core'

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

const parseDeviceInfo = (data: unknown, defaults?: Partial<DeviceInfo>): DeviceInfo | null => {
  if (!data || typeof data !== 'object') {
    if (!defaults) {
      return null
    }
    const hasDefault = Object.values(defaults).some((value) => typeof value === 'string' && value.length > 0)
    return hasDefault
      ? {
          pin_number: defaults.pin_number ?? '',
          ip: defaults.ip ?? '',
          mac: defaults.mac ?? '',
          cpe_id: defaults.cpe_id ?? '',
          host_id: defaults.host_id ?? ''
        }
      : null
  }
  const obj = data as Record<string, unknown>
  const hasAnyKey = [
    'pin_number',
    'pin',
    'ip',
    'mac',
    'cpe_id',
    'cpeId',
    'host_id',
    'hostId'
  ].some((key) => key in obj)
  if (!hasAnyKey && !defaults) {
    return null
  }
  const pin_number = (obj.pin_number ?? obj.pin ?? '') as string
  const ip = (obj.ip ?? '') as string
  const mac = (obj.mac ?? '') as string
  const cpe_id = (obj.cpe_id ?? obj.cpeId ?? '') as string
  const host_id = (obj.host_id ?? obj.hostId ?? '') as string

  return {
    pin_number: pin_number || defaults?.pin_number || '',
    ip: ip || defaults?.ip || '',
    mac: mac || defaults?.mac || '',
    cpe_id: cpe_id || defaults?.cpe_id || '',
    host_id: host_id || defaults?.host_id || ''
  }
}

const persistDeviceInfoIfPresent = async (res: unknown, defaults?: Partial<DeviceInfo>) => {
  let info = parseDeviceInfo(res, defaults)
  if (!info && typeof res === 'string') {
    try {
      info = parseDeviceInfo(JSON.parse(res), defaults)
    } catch {
      info = null
    }
  }
  if (!info) {
    return
  }
  await invoke('set_device_info', { payload: info })
}

const parseAuditPolicy = (data: unknown): string | null => {
  if (!data) {
    return null
  }
  if (typeof data === 'string') {
    return data.trim().length ? data : null
  }
  if (typeof data !== 'object') {
    return null
  }
  const obj = data as Record<string, unknown>
  const candidates = [
    obj.audit_policy,
    obj.audit_policy_json,
    obj.policy,
    obj.ebp_policy_update
  ]
  for (const candidate of candidates) {
    if (!candidate) {
      continue
    }
    if (typeof candidate === 'string') {
      return candidate.trim().length ? candidate : null
    }
    try {
      return JSON.stringify(candidate)
    } catch {
      continue
    }
  }
  return null
}

const persistAuditPolicyIfPresent = async (res: unknown) => {
  let policy = parseAuditPolicy(res)
  if (!policy && typeof res === 'string') {
    try {
      policy = parseAuditPolicy(JSON.parse(res))
    } catch {
      policy = null
    }
  }
  if (!policy) {
    return
  }
  await invoke('set_audit_policy', { payload: { policy_json: policy } })
}

const handleLogin = async () => {
  if (!loginForm.pin) {
    message.error('请输入 PIN 码')
    return
  }

  try {
    const res = await invoke('login', { payload: { pin: loginForm.pin } })
    message.success(res as string)
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
        ip: registerForm.ip, // Pass accurate IP
        mac: registerForm.mac // Pass accurate MAC
      }
    })
    message.success(res as string)
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
    // Store IP/MAC in the form for registration if needed
    Object.assign(registerForm, {
      ip: info.ip,
      mac: info.mac
    })
    
    // Also sync with audit service immediately if possible
    await invoke('set_device_info', { payload: info })
  } catch (err) {
    console.error('Failed to get system device info:', err)
  }
})
</script>

<template>
  <div class="login-container">
    <div class="glow-sphere s1"></div>
    <div class="glow-sphere s2"></div>
    
    <div class="login-glass-card">
      <div class="brand">
        <div class="logo-box">M</div>
        <h2>MAC MONITOR</h2>
        <p>终端安全审计与防护系统</p>
      </div>

      <a-tabs v-model:activeKey="activeKey" centered class="premium-tabs">
        <a-tab-pane key="login" tab="授权登录">
          <div class="form-body">
            <div class="input-group">
              <label>终端 PIN 码</label>
              <a-input-password 
                v-model:value="loginForm.pin" 
                placeholder="输入 6-12 位授权码"
                size="large"
                class="glass-input"
              />
            </div>
            <a-button type="primary" block size="large" class="submit-btn" @click="handleLogin">
              进入审计专网
            </a-button>
          </div>
        </a-tab-pane>
        
        <a-tab-pane key="register" tab="设备注册">
          <div class="form-body scrollable">
            <div class="input-group">
              <label>服务器地址</label>
              <a-input v-model:value="registerForm.serverIp" placeholder="IP 地址" class="glass-input" />
            </div>
            <div class="input-group">
              <label>端口</label>
              <a-input v-model:value="registerForm.serverPort" placeholder="8080" class="glass-input" />
            </div>
            <div class="input-group">
              <label>CPE ID</label>
              <a-input v-model:value="registerForm.cpeId" placeholder="设备唯一标识" class="glass-input" />
            </div>
            <div class="input-group">
              <label>授权 PIN</label>
              <a-input-password v-model:value="registerForm.pin" placeholder="注册授权码" class="glass-input" />
            </div>
            <a-button type="primary" block @click="handleRegister" class="submit-btn mt-10">
              完成设备绑定
            </a-button>
          </div>
        </a-tab-pane>
      </a-tabs>

      <div class="footer-note">
        安全等级: L3 | 本次连接已受加密保护
      </div>
    </div>
  </div>
</template>

<style scoped>
.login-container {
  width: 100vw;
  height: 100vh;
  background: #0f172a;
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
  overflow: hidden;
  font-family: 'Inter', sans-serif;
}

/* 动态光晕 */
.glow-sphere {
  position: absolute;
  width: 400px;
  height: 400px;
  border-radius: 50%;
  filter: blur(80px);
  z-index: 0;
  opacity: 0.4;
}
.s1 { background: #3b82f6; top: -100px; left: -100px; }
.s2 { background: #8b5cf6; bottom: -100px; right: -100px; }

/* 玻璃卡片 */
.login-glass-card {
  width: 420px;
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 24px;
  padding: 40px;
  z-index: 1;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
}

.brand {
  text-align: center;
  margin-bottom: 30px;
}

.logo-box {
  width: 48px;
  height: 48px;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  border-radius: 12px;
  margin: 0 auto 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 24px;
  font-weight: 800;
  box-shadow: 0 8px 16px rgba(59, 130, 246, 0.3);
}

.brand h2 {
  color: #fff;
  font-size: 22px;
  letter-spacing: 2px;
  margin: 0;
}

.brand p {
  color: #94a3b8;
  font-size: 13px;
  margin-top: 4px;
}

/* Tabs 样式 */
:deep(.ant-tabs-nav::before) { border-bottom: none !important; }
:deep(.ant-tabs-tab) { color: #64748b !important; font-size: 15px !important; }
:deep(.ant-tabs-tab-active .ant-tabs-tab-btn) { color: #3b82f6 !important; font-weight: 600; }
:deep(.ant-tabs-ink-bar) { background: #3b82f6 !important; height: 3px !important; border-radius: 3px; }

.form-body {
  padding: 20px 0 10px;
}

.input-group {
  margin-bottom: 20px;
}

.input-group label {
  display: block;
  color: #94a3b8;
  font-size: 12px;
  margin-bottom: 8px;
  padding-left: 4px;
}

.glass-input {
  background: rgba(255, 255, 255, 0.03) !important;
  border: 1px solid rgba(255, 255, 255, 0.1) !important;
  color: #fff !important;
  border-radius: 10px !important;
}

:deep(.ant-input-password-icon) { color: #64748b !important; }

.submit-btn {
  height: 48px !important;
  border-radius: 10px !important;
  background: linear-gradient(90deg, #3b82f6, #2563eb) !important;
  border: none !important;
  font-weight: 600 !important;
  box-shadow: 0 4px 12px rgba(37, 99, 235, 0.2) !important;
  margin-top: 10px;
}

.submit-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px rgba(37, 99, 235, 0.3) !important;
}

.footer-note {
  text-align: center;
  color: #475569;
  font-size: 11px;
  margin-top: 30px;
  padding-top: 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.05);
}

.mt-10 { margin-top: 10px; }
.scrollable { max-height: 300px; overflow-y: auto; padding-right: 5px; }

/* 滚动条美化 */
.scrollable::-webkit-scrollbar { width: 4px; }
.scrollable::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 4px; }
</style>
