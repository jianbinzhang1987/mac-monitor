<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryForm" size="small" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="序列号" prop="serialNumber">
        <el-input v-model="queryParams.serialNumber" placeholder="请输入设备序列号" clearable
          @keyup.enter.native="handleQuery" />
      </el-form-item>
      <el-form-item label="应用名称" prop="appName">
        <el-input v-model="queryParams.appName" placeholder="请输入应用名称" clearable @keyup.enter.native="handleQuery" />
      </el-form-item>
      <el-form-item label="应用包名" prop="bundleId">
        <el-input v-model="queryParams.bundleId" placeholder="请输入应用包名" clearable @keyup.enter.native="handleQuery" />
      </el-form-item>
      <el-form-item label="内容类型" prop="contentType">
        <el-select v-model="queryParams.contentType" placeholder="请选择内容类型" clearable>
          <el-option label="文本" value="text/plain" />
          <el-option label="链接" value="text/url" />
          <el-option label="图片" value="image" />
          <el-option label="RTF" value="application/rtf" />
        </el-select>
      </el-form-item>
      <el-form-item label="操作时间">
        <el-date-picker v-model="dateRange" style="width: 240px" value-format="yyyy-MM-dd" type="daterange"
          range-separator="-" start-placeholder="开始日期" end-placeholder="结束日期"></el-date-picker>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="el-icon-search" size="mini" @click="handleQuery">搜索</el-button>
        <el-button icon="el-icon-refresh" size="mini" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button type="danger" plain icon="el-icon-delete" size="mini" :disabled="multiple" @click="handleDelete"
          v-hasPermi="['monitor:log:clipboard:remove']">删除</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="warning" plain icon="el-icon-download" size="mini" @click="handleExport"
          v-hasPermi="['monitor:log:clipboard:export']">导出</el-button>
      </el-col>
      <right-toolbar :showSearch.sync="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="clipboardList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <el-table-column label="日志ID" align="center" prop="logId" />
      <el-table-column label="设备序列号" align="center" prop="serialNumber" width="120" />
      <el-table-column label="操作时间" align="center" prop="opTime" width="160">
        <template slot-scope="scope">
          <span>{{ parseTime(scope.row.opTime) }}</span>
        </template>
      </el-table-column>
      <el-table-column label="应用名称" align="center" prop="appName" />
      <el-table-column label="应用包名" align="center" prop="bundleId" />
      <el-table-column label="剪贴板内容" align="left" prop="content" :show-overflow-tooltip="true" />
      <el-table-column label="内容类型" align="center" prop="contentType">
        <template slot-scope="scope">
          <el-tag v-if="scope.row.contentType === 'text/plain'">文本</el-tag>
          <el-tag type="success" v-else-if="scope.row.contentType === 'text/url'">链接</el-tag>
          <el-tag type="warning" v-else-if="scope.row.contentType === 'image'">图片</el-tag>
          <el-tag type="info" v-else>{{ scope.row.contentType }}</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="风险等级" align="center" prop="riskLevel">
        <template slot-scope="scope">
          <el-tag type="danger" v-if="scope.row.riskLevel >= 2">高风险</el-tag>
          <el-tag type="warning" v-else-if="scope.row.riskLevel == 1">中风险</el-tag>
          <el-tag type="success" v-else>低风险</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="主机ID" align="center" prop="hostId" />
      <el-table-column label="IP地址" align="center" prop="ip" />
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template slot-scope="scope">
          <el-button size="mini" type="text" icon="el-icon-delete" @click="handleDelete(scope.row)"
            v-hasPermi="['monitor:log:clipboard:remove']">删除</el-button>
          <el-button size="mini" type="text" icon="el-icon-view" @click="handleViewContent(scope.row)">详情</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog :title="title" :visible.sync="openContent" width="600px" append-to-body>
      <div style="white-space: pre-wrap; word-break: break-all; max-height: 500px; overflow-y: auto;">
        {{ contentDetail }}
      </div>
      <div slot="footer" class="dialog-footer">
        <el-button @click="openContent = false">关 闭</el-button>
      </div>
    </el-dialog>

    <pagination v-show="total > 0" :total="total" :page.sync="queryParams.pageNum" :limit.sync="queryParams.pageSize"
      @pagination="getList" />
  </div>
</template>

<script>
import { listClipboard, getClipboard, delClipboard, addClipboard, updateClipboard } from "@/api/monitor/clipboard";

export default {
  name: "Clipboard",
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
      // 显示内容详情弹窗
      openContent: false,
      // 详情内容
      contentDetail: "",
      // 显示搜索条件
      showSearch: true,
      // 总条数
      total: 0,
      // 剪贴板审计日志表格数据
      clipboardList: [],
      // 弹出层标题
      title: "",
      // 是否显示弹出层
      open: false,
      // 日期范围
      dateRange: [],
      // 查询参数
      queryParams: {
        pageNum: 1,
        pageSize: 10,
        serialNumber: null,
        deviceId: null,
        opTime: null,
        appName: null,
        bundleId: null,
        content: null,
        contentType: null,
        riskLevel: null,
        hostId: null,
        mac: null,
        ip: null,
      },
      // 表单参数
      form: {},
      // 表单校验
      rules: {
      }
    };
  },
  created() {
    this.getList();
  },
  methods: {
    /** 查询剪贴板审计日志列表 */
    getList() {
      this.loading = true;
      listClipboard(this.addDateRange(this.queryParams, this.dateRange)).then(response => {
        this.clipboardList = response.rows;
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
        logId: null,
        serialNumber: null,
        deviceId: null,
        opTime: null,
        appName: null,
        bundleId: null,
        content: null,
        contentType: null,
        riskLevel: null,
        hostId: null,
        mac: null,
        ip: null,
        createTime: null
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
      this.dateRange = [];
      this.resetForm("queryForm");
      this.handleQuery();
    },
    // 多选框选中数据
    handleSelectionChange(selection) {
      this.ids = selection.map(item => item.logId)
      this.single = selection.length !== 1
      this.multiple = !selection.length
    },
    /** 删除按钮操作 */
    handleDelete(row) {
      const logIds = row.logId || this.ids;
      this.$modal.confirm('是否确认删除剪贴板审计日志编号为"' + logIds + '"的数据项？').then(function () {
        return delClipboard(logIds);
      }).then(() => {
        this.getList();
        this.$modal.msgSuccess("删除成功");
      }).catch(() => { });
    },
    /** 导出按钮操作 */
    handleExport() {
      this.download('monitor/log/clipboard/export', {
        ...this.queryParams
      }, `clipboard_${new Date().getTime()}.xlsx`)
    },
    /** 查看详情操作 */
    handleViewContent(row) {
      this.contentDetail = row.content;
      this.title = "剪贴板详情";
      this.openContent = true;
    }
  }
};
</script>
