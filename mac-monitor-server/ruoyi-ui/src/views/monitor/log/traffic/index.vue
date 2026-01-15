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
      <el-form-item label="域名" prop="domain">
        <el-input
          v-model="queryParams.domain"
          placeholder="请输入域名"
          clearable
          @keyup.enter.native="handleQuery"
        />
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
          v-hasPermi="['monitor:log:traffic:export']"
        >导出</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="danger"
          plain
          icon="el-icon-delete"
          size="mini"
          :disabled="multiple"
          @click="handleDelete"
          v-hasPermi="['monitor:log:traffic:remove']"
        >删除</el-button>
      </el-col>
      <right-toolbar :showSearch.sync="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="trafficList" @selection-change="handleSelectionChange" border>
      <el-table-column type="selection" width="55" align="center" />
      <el-table-column label="设备序列号" align="center" prop="serialNumber" width="150" />
      <el-table-column label="审计时间" align="center" prop="auditTime" width="160">
        <template slot-scope="scope">
          <span>{{ parseTime(scope.row.auditTime) }}</span>
        </template>
      </el-table-column>
      <el-table-column label="域名" align="center" prop="domain" width="180" :show-overflow-tooltip="true" />
      <el-table-column label="完整 URL" align="left" prop="url" :show-overflow-tooltip="true" />
      <el-table-column label="进程" align="center" prop="processName" width="120" />
      <el-table-column label="风险等级" align="center" prop="riskLevel" width="100">
        <template slot-scope="scope">
          <el-tag v-if="scope.row.riskLevel == 0" type="success" effect="plain">低风险</el-tag>
          <el-tag v-else-if="scope.row.riskLevel == 1" type="warning" effect="dark">中风险</el-tag>
          <el-tag v-else type="danger" effect="dark">高风险</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width" width="150">
        <template slot-scope="scope">
          <el-button
            size="mini"
            type="text"
            icon="el-icon-plus"
            @click="handleAddWhitelist(scope.row)"
            v-hasPermi="['monitor:policy:edit']"
          >加入白名单</el-button>
          <el-button
            size="mini"
            type="text"
            icon="el-icon-delete"
            @click="handleDelete(scope.row)"
            v-hasPermi="['monitor:log:traffic:remove']"
          >删除</el-button>
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

    <!-- 加入白名单对话框 -->
    <el-dialog title="将域名加入关联策略白名单" :visible.sync="whitelistOpen" width="500px" append-to-body>
      <el-form ref="whitelistForm" :model="whitelistForm" label-width="100px">
        <el-form-item label="设备序列号">
          <el-input v-model="whitelistForm.serialNumber" readonly />
        </el-form-item>
        <el-form-item label="待加入域名">
          <el-input v-model="whitelistForm.domain" readonly />
        </el-form-item>
        <div style="padding-left: 100px; color: #909399; font-size: 13px; margin-bottom: 20px;">
          加入白名单后，该域名下的流量将不再进行敏感信息审计。
        </div>
      </el-form>
      <div slot="footer" class="dialog-footer">
        <el-button type="primary" @click="submitWhitelist">确 认</el-button>
        <el-button @click="whitelistOpen = false">取 消</el-button>
      </div>
    </el-dialog>
  </div>
</template>

<script>
import { listTraffic, delTraffic } from "@/api/monitor/traffic";
import { listPolicy, getPolicy, updatePolicy } from "@/api/monitor/policy";
import { listDevice } from "@/api/monitor/device";

export default {
  name: "TrafficLog",
  data() {
    return {
      // 遮罩层
      loading: true,
      // 选中数组
      ids: [],
      // 非多个禁用
      multiple: true,
      // 显示搜索条件
      showSearch: true,
      // 总条数
      total: 0,
      // 流量日志表格数据
      trafficList: [],
      // 加入白名单相关
      whitelistOpen: false,
      whitelistForm: {
        serialNumber: '',
        domain: ''
      },
      // 查询参数
      queryParams: {
        pageNum: 1,
        pageSize: 10,
        serialNumber: null,
        domain: null,
        riskLevel: null
      }
    };
  },
  created() {
    this.getList();
  },
  methods: {
    /** 查询流量日志列表 */
    getList() {
      this.loading = true;
      listTraffic(this.queryParams).then(response => {
        this.trafficList = response.rows;
        this.total = response.total;
        this.loading = false;
      });
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
      this.ids = selection.map(item => item.logId)
      this.multiple = !selection.length
    },
    /** 加入白名单按钮操作 */
    handleAddWhitelist(row) {
      this.whitelistForm = {
        serialNumber: row.serialNumber,
        domain: row.domain
      };
      this.whitelistOpen = true;
    },
    /** 提交白名单 */
    submitWhitelist() {
      this.loading = true;
      // 1. 查找设备关联策略
      listDevice({ serialNumber: this.whitelistForm.serialNumber }).then(response => {
        let policyPromise;
        if (response.rows && response.rows.length > 0 && response.rows[0].policyId) {
          policyPromise = getPolicy(response.rows[0].policyId);
        } else {
          policyPromise = listPolicy({ isDefault: '1' }).then(res => {
            return res.rows && res.rows.length > 0 ? { data: res.rows[0] } : Promise.reject("未找到策略");
          });
        }

        policyPromise.then(res => {
          const policy = res.data;
          let trafficRules = {};
          try {
            trafficRules = JSON.parse(policy.trafficRules || '{}');
          } catch (e) {
            trafficRules = {};
          }

          if (!trafficRules.whitelist) {
            trafficRules.whitelist = [];
          }

          if (!trafficRules.whitelist.includes(this.whitelistForm.domain)) {
            trafficRules.whitelist.push(this.whitelistForm.domain);
            policy.trafficRules = JSON.stringify(trafficRules);

            updatePolicy(policy).then(() => {
              this.$modal.msgSuccess(`域名 [${this.whitelistForm.domain}] 已成功加入策略 [${policy.policyName}] 的白名单`);
              this.whitelistOpen = false;
              this.loading = false;
            });
          } else {
            this.$modal.msgWarning("该域名已在白名单中");
            this.whitelistOpen = false;
            this.loading = false;
          }
        }).catch(err => {
          this.$modal.msgError("操作失败: " + err);
          this.loading = false;
        });
      });
    },
    /** 删除按钮操作 */
    handleDelete(row) {
      const logIds = row.logId || this.ids;
      this.$modal.confirm('是否确认删除流量审计日志编号为"' + logIds + '"的数据项？').then(function() {
        return delTraffic(logIds);
      }).then(() => {
        this.getList();
        this.$modal.msgSuccess("删除成功");
      }).catch(() => {});
    },
    /** 导出按钮操作 */
    handleExport() {
      this.download('monitor/log/traffic/export', {
        ...this.queryParams
      }, `traffic_${new Date().getTime()}.xlsx`)
    }
  }
};
</script>
