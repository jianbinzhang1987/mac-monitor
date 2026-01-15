<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryForm" size="small" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="设备号" prop="serialNumber">
        <el-input
          v-model="queryParams.serialNumber"
          placeholder="请输入设备序列号"
          clearable
          @keyup.enter.native="handleQuery"
        />
      </el-form-item>
      <el-form-item label="事件类型" prop="eventType">
        <el-select v-model="queryParams.eventType" placeholder="请选择类型" clearable>
          <el-option label="进程启动" value="PROCESS_LAUNCH" />
          <el-option label="USB插入" value="USB_INSERT" />
          <el-option label="文件变更" value="FILE_CHANGE" />
        </el-select>
      </el-form-item>
      <el-form-item label="风险等级" prop="riskLevel">
        <el-select v-model="queryParams.riskLevel" placeholder="请选择风险等级" clearable>
          <el-option label="低" value="0" />
          <el-option label="中" value="1" />
          <el-option label="高" value="2" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="el-icon-search" size="mini" @click="handleQuery">搜索</el-button>
        <el-button icon="el-icon-refresh" size="mini" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button
          type="warning"
          plain
          icon="el-icon-download"
          size="mini"
          @click="handleExport"
          v-hasPermi="['monitor:log:behavior:export']"
        >导出</el-button>
      </el-col>
      <right-toolbar :showSearch.sync="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="behaviorList" border>
      <el-table-column label="设备序列号" align="center" prop="serialNumber" width="150" />
      <el-table-column label="事件时间" align="center" prop="eventTime" width="160">
        <template slot-scope="scope">
          <span>{{ parseTime(scope.row.eventTime) }}</span>
        </template>
      </el-table-column>
      <el-table-column label="事件类型" align="center" prop="eventType" width="120">
        <template slot-scope="scope">
          <el-tag size="small">{{ formatEventType(scope.row.eventType) }}</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="相关进程" align="center" prop="processName" width="150" />
      <el-table-column label="详情描述" align="left" prop="detail">
        <template slot-scope="scope">
          <div class="detail-container">
            {{ formatDetail(scope.row.detail) }}
          </div>
        </template>
      </el-table-column>
      <el-table-column label="风险等级" align="center" prop="riskLevel" width="100">
        <template slot-scope="scope">
          <el-tag v-if="scope.row.riskLevel == 0" type="success">低风险</el-tag>
          <el-tag v-else-if="scope.row.riskLevel == 1" type="warning">中风险</el-tag>
          <el-tag v-else type="danger">高风险</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width" width="100">
        <template slot-scope="scope">
          <el-button
            size="mini"
            type="text"
            icon="el-icon-delete"
            @click="handleDelete(scope.row)"
            v-hasPermi="['monitor:log:behavior:remove']"
          >删除</el-button>
          <el-button
            v-if="scope.row.eventType === 'PROCESS_LAUNCH'"
            size="mini"
            type="text"
            icon="el-icon-lock"
            @click="handleBlockProcess(scope.row)"
            v-hasPermi="['monitor:policy:edit']"
          >禁止进程</el-button>
          <el-button
            v-if="scope.row.eventType === 'USB_INSERT'"
            size="mini"
            type="text"
            icon="el-icon-circle-close"
            @click="handleDisableUsb(scope.row)"
            v-hasPermi="['monitor:policy:edit']"
          >禁用USB</el-button>
        </template>
      </el-table-column>
    </el-table>

    <pagination
      v-show="total>0"
      :total="total"
      :page.sync="queryParams.pageNum"
      :limit.sync="queryParams.pageSize"
      @pagination="getList"
    />
  </div>
</template>

<script>
import { listBehavior, delBehavior } from "@/api/monitor/behavior";
import { quickAddRule } from "@/api/monitor/policy";
import { getDevice } from "@/api/monitor/device";

export default {
  name: "BehaviorLog",
  data() {
    return {
      // 遮罩层
      loading: true,
      // 显示搜索条件
      showSearch: true,
      // 总条数
      total: 0,
      // 行为日志表格数据
      behaviorList: [],
      // 查询参数
      queryParams: {
        pageNum: 1,
        pageSize: 10,
        serialNumber: null,
        eventType: null,
        riskLevel: null
      }
    };
  },
  created() {
    this.getList();
  },
  methods: {
    /** 查询行为日志列表 */
    getList() {
      this.loading = true;
      listBehavior(this.queryParams).then(response => {
        this.behaviorList = response.rows;
        this.total = response.total;
        this.loading = false;
      });
    },
    /** 格式化事件类型 */
    formatEventType(type) {
      const maps = {
        'PROCESS_LAUNCH': '进程启动',
        'USB_INSERT': 'USB插入',
        'FILE_CHANGE': '文件变更'
      };
      return maps[type] || type;
    },
    /** 格式化详情内容 */
    formatDetail(detail) {
      try {
        if (typeof detail === 'string' && (detail.startsWith('{') || detail.startsWith('['))) {
          const obj = JSON.parse(detail);
          return JSON.stringify(obj, null, 2);
        }
      } catch (e) {}
      return detail;
    },
    /** 搜索按钮操作 */
    handleQuery() {
      this.queryParams.pageNum = 1;
      this.getList();
    },
    /** 重置按钮操作 */
    resetQuery() {
      this.resetForm("queryForm");
      this.handleQuery();
    },
    /** 删除按钮操作 */
    handleDelete(row) {
      const logIds = row.logId;
      this.$modal.confirm('是否确认删除行为审计日志编号为"' + logIds + '"的数据项？').then(function() {
        return delBehavior(logIds);
      }).then(() => {
        this.getList();
        this.$modal.msgSuccess("删除成功");
      }).catch(() => {});
    },
    /** 禁止进程操作 */
    handleBlockProcess(row) {
      const processName = row.processName;
      if (!processName) {
        this.$modal.msgError("无法获取进程名称");
        return;
      }
      this.$modal.confirm('是否确认禁止运行进程："' + processName + '"？此操作将更新设备当前的关联策略。').then(() => {
        // 先获取设备当前关联的策略ID
        return getDevice(row.deviceId);
      }).then(response => {
        const policyId = response.data.policyId;
        if (!policyId) {
          throw new Error("设备未关联特定策略，无法直接修改。请先在设备管理中指派策略。");
        }
        return quickAddRule({
          policyId: policyId,
          remark: 'PROCESS_BLOCK', // 借用字段传递类型
          trafficRules: processName // 借用字段传递值
        });
      }).then(() => {
        this.$modal.msgSuccess("已成功将进程加入黑名单");
      }).catch(err => {
        if (err && err.message) {
          this.$modal.msgError(err.message);
        }
      });
    },
    /** 禁用USB操作 */
    handleDisableUsb(row) {
      this.$modal.confirm('是否确认在策略中禁用所有USB存储设备？此操作将影响该策略下的所有设备。').then(() => {
        return getDevice(row.deviceId);
      }).then(response => {
        const policyId = response.data.policyId;
        if (!policyId) {
          throw new Error("设备未关联特定策略，无法直接修改。");
        }
        return quickAddRule({
          policyId: policyId,
          remark: 'USB_DISABLE',
          trafficRules: ''
        });
      }).then(() => {
        this.$modal.msgSuccess("已成功更新策略：禁用USB");
      }).catch(err => {
        if (err && err.message) {
          this.$modal.msgError(err.message);
        }
      });
    },
    /** 导出按钮操作 */
    handleExport() {
      this.download('monitor/log/behavior/export', {
        ...this.queryParams
      }, `behavior_${new Date().getTime()}.xlsx`)
    }
  }
};
</script>

<style scoped>
.detail-container {
    max-height: 80px;
    overflow-y: auto;
    font-size: 12px;
    color: #606266;
    background-color: #f8f9fa;
    padding: 5px;
    border-radius: 4px;
    white-space: pre-wrap;
    word-break: break-all;
}
</style>
