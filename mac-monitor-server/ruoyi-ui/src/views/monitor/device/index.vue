<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryForm" size="small" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="序列号" prop="serialNumber">
        <el-input
          v-model="queryParams.serialNumber"
          placeholder="请输入序列号"
          clearable
          @keyup.enter.native="handleQuery"
        />
      </el-form-item>
      <el-form-item label="设备名称" prop="deviceName">
        <el-input
          v-model="queryParams.deviceName"
          placeholder="请输入设备名称"
          clearable
          @keyup.enter.native="handleQuery"
        />
      </el-form-item>
      <el-form-item label="在线状态" prop="status">
        <el-select v-model="queryParams.status" placeholder="请选择在线状态" clearable>
          <el-option label="离线" value="0" />
          <el-option label="在线" value="1" />
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
          type="primary"
          plain
          icon="el-icon-plus"
          size="mini"
          @click="handleAdd"
          v-hasPermi="['monitor:device:add']"
        >新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="success"
          plain
          icon="el-icon-edit"
          size="mini"
          :disabled="single"
          @click="handleUpdate"
          v-hasPermi="['monitor:device:edit']"
        >修改</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="danger"
          plain
          icon="el-icon-delete"
          size="mini"
          :disabled="multiple"
          @click="handleDelete"
          v-hasPermi="['monitor:device:remove']"
        >删除</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="info"
          plain
          icon="el-icon-setting"
          size="mini"
          :disabled="multiple"
          @click="handleBatchPolicy"
          v-hasPermi="['monitor:device:edit']"
        >批量更换策略</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="warning"
          plain
          icon="el-icon-download"
          size="mini"
          @click="handleExport"
          v-hasPermi="['monitor:device:export']"
        >导出</el-button>
      </el-col>
      <right-toolbar :showSearch.sync="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="deviceList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <el-table-column label="序列号" align="center" prop="serialNumber" width="150" />
      <el-table-column label="设备名称" align="center" prop="deviceName" />
      <el-table-column label="在线状态" align="center" prop="status" width="100">
        <template slot-scope="scope">
          <el-tag v-if="scope.row.status === '1'" type="success">在线</el-tag>
          <el-tag v-else type="info">离线</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="系统/客户端" align="center" width="180">
        <template slot-scope="scope">
          <div>{{ scope.row.osVersion }}</div>
          <div style="font-size: 12px; color: #909399;">v{{ scope.row.appVersion }}</div>
        </template>
      </el-table-column>
      <el-table-column label="策略版本" align="center" prop="policyVersion">
        <template slot-scope="scope">
          <el-tooltip class="item" effect="dark" :content="'目标版本: ' + (scope.row.targetVersion || '未指派')" placement="top">
            <el-tag :type="scope.row.policyVersion === String(scope.row.targetVersion) ? 'success' : 'warning'">
              {{ scope.row.policyVersion || '未知' }}
              <i v-if="scope.row.policyVersion !== String(scope.row.targetVersion)" class="el-icon-refresh"></i>
            </el-tag>
          </el-tooltip>
        </template>
      </el-table-column>
      <el-table-column label="最后心跳" align="center" prop="lastHeartbeat" width="160">
        <template slot-scope="scope">
          <span>{{ parseTime(scope.row.lastHeartbeat) }}</span>
        </template>
      </el-table-column>
      <el-table-column label="注册IP" align="center" prop="registeredIp" />
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template slot-scope="scope">
          <el-button
            size="mini"
            type="text"
            icon="el-icon-view"
            @click="handleViewPolicy(scope.row)"
          >策略</el-button>
          <el-button
            size="mini"
            type="text"
            icon="el-icon-edit"
            @click="handleUpdate(scope.row)"
            v-hasPermi="['monitor:device:edit']"
          >修改</el-button>
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

    <!-- 添加或修改监控设备对话框 -->
    <el-dialog :title="title" :visible.sync="open" width="500px" append-to-body>
      <el-form ref="form" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="序列号" prop="serialNumber">
          <el-input v-model="form.serialNumber" placeholder="请输入序列号" />
        </el-form-item>
        <el-form-item label="设备名称" prop="deviceName">
          <el-input v-model="form.deviceName" placeholder="请输入设备名称" />
        </el-form-item>
        <el-form-item label="关联策略" prop="policyId">
          <el-select v-model="form.policyId" placeholder="请选择关联策略" style="width: 100%">
            <el-option
              v-for="item in policyOptions"
              :key="item.policyId"
              :label="item.policyName"
              :value="item.policyId"
            >
              <span style="float: left">{{ item.policyName }}</span>
              <span style="float: right; color: #8492a6; font-size: 13px">v{{ item.version }}</span>
            </el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="备注" prop="remark">
          <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" />
        </el-form-item>
      </el-form>
      <div slot="footer" class="dialog-footer">
        <el-button type="primary" @click="submitForm">确 定</el-button>
        <el-button @click="cancel">取 消</el-button>
      </div>
    </el-dialog>

    <!-- 批量更换策略对话框 -->
    <el-dialog title="批量更换策略" :visible.sync="batchPolicyOpen" width="400px" append-to-body>
      <el-form label-width="80px">
        <el-form-item label="选择策略">
          <el-select v-model="batchPolicyId" placeholder="请选择要指派的策略" style="width: 100%">
            <el-option
              v-for="item in policyOptions"
              :key="item.policyId"
              :label="item.policyName"
              :value="item.policyId"
            >
              <span style="float: left">{{ item.policyName }}</span>
              <span style="float: right; color: #8492a6; font-size: 13px">v{{ item.version }}</span>
            </el-option>
          </el-select>
        </el-form-item>
      </el-form>
      <div slot="footer" class="dialog-footer">
        <el-button type="primary" @click="submitBatchPolicy">确 定</el-button>
        <el-button @click="batchPolicyOpen = false">取 消</el-button>
      </div>
    </el-dialog>

    <!-- 策略预览对话框 -->
    <el-dialog title="当前生效策略预览" :visible.sync="policyPreviewOpen" width="600px" append-to-body>
      <div v-loading="policyLoading">
        <el-descriptions :column="1" border>
          <el-descriptions-item label="策略名称">{{ currentPolicy.policyName }}</el-descriptions-item>
          <el-descriptions-item label="策略版本">
            <el-tag size="small">{{ currentPolicy.version }}</el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="流量规则">
            <div v-if="parsedRules.traffic">
              <div><strong>白名单:</strong> {{ (parsedRules.traffic.whitelist && parsedRules.traffic.whitelist.join(', ')) || '无' }}</div>
              <div><strong>敏感词:</strong> {{ (parsedRules.traffic.keywords && parsedRules.traffic.keywords.join(', ')) || '无' }}</div>
            </div>
          </el-descriptions-item>
          <el-descriptions-item label="截屏规则">
            <div v-if="parsedRules.screenshot">
              <div><strong>间隔:</strong> {{ parsedRules.screenshot.interval }} 秒</div>
              <div><strong>OCR:</strong> {{ parsedRules.screenshot.ocr_enable ? '开启' : '关闭' }}</div>
              <div v-if="parsedRules.screenshot.apps && parsedRules.screenshot.apps.length">
                <strong>定向应用:</strong> {{ parsedRules.screenshot.apps && parsedRules.screenshot.apps.join(', ') }}
              </div>
            </div>
          </el-descriptions-item>
        </el-descriptions>
        <div style="margin-top: 20px; color: #E6A23C; font-size: 13px;">
          <i class="el-icon-info"></i> 提示：所有设备默认应用系统标记为“默认”的策略。若需修改规则，请前往策略管理。
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<script>
import { listDevice, getDevice, delDevice, addDevice, updateDevice, updateDevicePolicy } from "@/api/monitor/device";
import { listPolicy } from "@/api/monitor/policy";

export default {
  name: "Device",
  data() {
    return {
      // 遮罩层
      loading: true,
      // 选中数组
      ids: [],
      // 非单个禁用
      single: true,
      // 非多个禁用
      multiple: true,
      // 显示搜索条件
      showSearch: true,
      // 总条数
      total: 0,
      // 监控设备表格数据
      deviceList: [],
      // 策略选项
      policyOptions: [],
      // 最新策略版本
      latestPolicyVersion: '1',
      // 弹出层标题
      title: "",
      // 是否显示弹出层
      open: false,
      // 批量指派策略弹出层
      batchPolicyOpen: false,
      batchPolicyId: null,
      // 查询参数
      queryParams: {
        pageNum: 1,
        pageSize: 10,
        serialNumber: null,
        deviceName: null,
        status: null,
      },
      // 表单参数
      form: {},
      // 策略预览相关
      policyPreviewOpen: false,
      policyLoading: false,
      currentPolicy: {},
      parsedRules: {
        traffic: null,
        screenshot: null
      },
      // 表单校验
      rules: {
        serialNumber: [
          { required: true, message: "序列号不能为空", trigger: "blur" }
        ],
      }
    };
  },
  created() {
    this.getList();
    this.getLatestPolicy();
    this.getPolicyOptions();
  },
  methods: {
    /** 查询策略选项列表 */
    getPolicyOptions() {
      listPolicy().then(response => {
        this.policyOptions = response.rows;
      });
    },
    /** 查询最新默认策略版本 */
    getLatestPolicy() {
      listPolicy({ isDefault: '1' }).then(response => {
        if (response.rows && response.rows.length > 0) {
          this.latestPolicyVersion = response.rows[0].version;
        }
      });
    },
    /** 查看设备策略详情 */
    handleViewPolicy(row) {
      this.policyPreviewOpen = true;
      this.policyLoading = true;

      const getPolicyPromise = row.policyId
        ? getDevice(row.deviceId).then(res => {
            // 重新获取详情确保拿到最新的 policyId
            if (res.data.policyId) {
              return listPolicy({ policyId: res.data.policyId });
            }
            return listPolicy({ isDefault: '1' });
          })
        : listPolicy({ isDefault: '1' });

      getPolicyPromise.then(response => {
        if (response.rows && response.rows.length > 0) {
          this.currentPolicy = response.rows[0];
          try {
            this.parsedRules.traffic = JSON.parse(this.currentPolicy.trafficRules || '{}');
            this.parsedRules.screenshot = JSON.parse(this.currentPolicy.screenshotRules || '{}');
          } catch (e) {
            this.parsedRules = { traffic: null, screenshot: null };
          }
        } else {
          this.currentPolicy = { policyName: '未找到策略' };
          this.parsedRules = { traffic: null, screenshot: null };
        }
        this.policyLoading = false;
      }).catch(() => {
        this.policyLoading = false;
      });
    },
    /** 查询监控设备列表 */
    getList() {
      this.loading = true;
      listDevice(this.queryParams).then(response => {
        this.deviceList = response.rows;
        this.total = response.total;
        this.loading = false;
      });
    },
    // 取消按钮
    cancel() {
      this.open = false;
      this.reset();
    },
    // 表单重置
    reset() {
      this.form = {
        deviceId: null,
        serialNumber: null,
        deviceName: null,
        osVersion: null,
        appVersion: null,
        status: "0",
        lastHeartbeat: null,
        policyVersion: null,
        registeredIp: null,
        createBy: null,
        createTime: null,
        updateBy: null,
        updateTime: null,
        remark: null
      };
      this.resetForm("form");
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
    // 多选框选中数据
    handleSelectionChange(selection) {
      this.ids = selection.map(item => item.deviceId)
      this.single = selection.length!==1
      this.multiple = !selection.length
    },
    /** 新增按钮操作 */
    handleAdd() {
      this.reset();
      this.open = true;
      this.title = "添加监控设备";
    },
    /** 修改按钮操作 */
    handleUpdate(row) {
      this.reset();
      const deviceId = row.deviceId || this.ids
      getDevice(deviceId).then(response => {
        this.form = response.data;
        this.open = true;
        this.title = "修改监控设备";
      });
    },
    /** 批量更换策略按钮操作 */
    handleBatchPolicy() {
      this.batchPolicyId = null;
      this.batchPolicyOpen = true;
    },
    /** 提交批量更换策略 */
    submitBatchPolicy() {
      if (!this.batchPolicyId) {
        this.$modal.msgError("请选择策略");
        return;
      }
      this.$modal.confirm('是否确认向选中的 ' + this.ids.length + ' 台设备批量指派新策略？').then(() => {
        return updateDevicePolicy(this.ids, this.batchPolicyId);
      }).then(() => {
        this.batchPolicyOpen = false;
        this.getList();
        this.$modal.msgSuccess("批量指派成功");
      }).catch(() => {});
    },
    /** 提交按钮 */
    submitForm() {
      this.$refs["form"].validate(valid => {
        if (valid) {
          if (this.form.deviceId != null) {
            updateDevice(this.form).then(response => {
              this.$modal.msgSuccess("修改成功");
              this.open = false;
              this.getList();
            });
          } else {
            addDevice(this.form).then(response => {
              this.$modal.msgSuccess("新增成功");
              this.open = false;
              this.getList();
            });
          }
        }
      });
    },
    /** 删除按钮操作 */
    handleDelete(row) {
      const deviceIds = row.deviceId || this.ids;
      this.$modal.confirm('是否确认删除监控设备编号为"' + deviceIds + '"的数据项？').then(function() {
        return delDevice(deviceIds);
      }).then(() => {
        this.getList();
        this.$modal.msgSuccess("删除成功");
      }).catch(() => {});
    },
    /** 导出按钮操作 */
    handleExport() {
      this.download('monitor/device/export', {
        ...this.queryParams
      }, `device_${new Date().getTime()}.xlsx`)
    }
  }
};
</script>
