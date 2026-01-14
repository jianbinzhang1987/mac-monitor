<script setup lang="ts">
import { ref } from 'vue';
import {
  PieChartOutlined,
  DesktopOutlined,
  UserOutlined,
  TeamOutlined,
  FileOutlined,
  SafetyCertificateOutlined,
  CameraOutlined
} from '@ant-design/icons-vue';

const collapsed = ref<boolean>(false);
const selectedKeys = ref<string[]>(['1']);

// 模拟数据
const systemStatus = ref({
  serviceRunning: true,
  auditCount: 128,
  riskCount: 2,
  lastSync: '10:23:45'
});
</script>

<template>
  <a-layout style="min-height: 100vh">
    <a-layout-sider v-model:collapsed="collapsed" collapsible>
      <div class="logo">
        <span v-if="!collapsed" style="color: white; font-weight: bold; font-size: 16px;">Mac Monitor</span>
        <span v-else style="color: white; font-weight: bold;">MM</span>
      </div>
      <a-menu v-model:selectedKeys="selectedKeys" theme="dark" mode="inline">
        <a-menu-item key="1">
          <pie-chart-outlined />
          <span>系统概览</span>
        </a-menu-item>
        <a-menu-item key="2">
          <desktop-outlined />
          <span>网络流量</span>
        </a-menu-item>
        <a-sub-menu key="sub1">
          <template #title>
            <span>
              <user-outlined />
              <span>行为审计</span>
            </span>
          </template>
          <a-menu-item key="3"><safety-certificate-outlined /> 进程防护</a-menu-item>
          <a-menu-item key="4"><file-outlined /> 文件操作</a-menu-item>
          <a-menu-item key="5"><camera-outlined /> 屏幕截图</a-menu-item>
        </a-sub-menu>
        <a-menu-item key="9">
          <team-outlined />
          <span>设置</span>
        </a-menu-item>
      </a-menu>
    </a-layout-sider>
    <a-layout>
      <a-layout-header style="background: #fff; padding: 0 24px; display: flex; justify-content: space-between; align-items: center;">
        <h3>终端安全审计系统</h3>
        <div>
          <span style="margin-right: 15px;">当前用户: Employee_001</span>
          <a-button type="link" danger>退出登录</a-button>
        </div>
      </a-layout-header>
      <a-layout-content style="margin: 0 16px">
        <a-breadcrumb style="margin: 16px 0">
          <a-breadcrumb-item>首页</a-breadcrumb-item>
          <a-breadcrumb-item>概览</a-breadcrumb-item>
        </a-breadcrumb>
        <div :style="{ padding: '24px', background: '#fff', minHeight: '360px' }">

          <!-- 概览页面内容 -->
          <div v-if="selectedKeys[0] === '1'">
            <a-row :gutter="16">
              <a-col :span="6">
                <a-card>
                  <a-statistic title="服务状态" :value="systemStatus.serviceRunning ? '运行中' : '已停止'" :value-style="{ color: '#3f8600' }">
                    <template #prefix>
                      <safety-certificate-outlined />
                    </template>
                  </a-statistic>
                </a-card>
              </a-col>
              <a-col :span="6">
                <a-card>
                  <a-statistic title="今日审计日志" :value="systemStatus.auditCount" />
                </a-card>
              </a-col>
              <a-col :span="6">
                <a-card>
                  <a-statistic title="风险阻断" :value="systemStatus.riskCount" :value-style="{ color: '#cf1322' }" />
                </a-card>
              </a-col>
              <a-col :span="6">
                <a-card>
                  <a-statistic title="上次同步" :value="systemStatus.lastSync" />
                </a-card>
              </a-col>
            </a-row>

            <a-divider orientation="left">实时活动</a-divider>
            <a-list item-layout="horizontal" :data-source="[]">
              <template #renderItem="{ item }">
                <a-list-item>
                  <a-list-item-meta description="暂无最新活动数据">
                    <template #title>
                      <a href="https://www.antdv.com/">等待数据...</a>
                    </template>
                  </a-list-item-meta>
                </a-list-item>
              </template>
            </a-list>
          </div>

          <!-- 占位符 -->
          <div v-else style="text-align: center; padding-top: 50px;">
            <a-empty description="该模块正在开发中" />
          </div>

        </div>
      </a-layout-content>
      <a-layout-footer style="text-align: center">
        Mac Monitor System ©2026 Created by Internal Security Team
      </a-layout-footer>
    </a-layout>
  </a-layout>
</template>

<style scoped>
.logo {
  height: 32px;
  margin: 16px;
  background: rgba(255, 255, 255, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
}
</style>