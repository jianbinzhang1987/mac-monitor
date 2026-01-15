<template>
    <div class="app-container">
        <el-form :model="queryParams" ref="queryForm" size="small" :inline="true" v-show="showSearch"
            label-width="68px">
            <el-form-item label="策略名称" prop="policyName">
                <el-input v-model="queryParams.policyName" placeholder="请输入策略名称" clearable
                    @keyup.enter.native="handleQuery" />
            </el-form-item>
            <el-form-item label="默认策略" prop="isDefault">
                <el-select v-model="queryParams.isDefault" placeholder="请选择" clearable>
                    <el-option label="是" value="1" />
                    <el-option label="否" value="0" />
                </el-select>
            </el-form-item>
            <el-form-item>
                <el-button type="primary" icon="el-icon-search" size="mini" @click="handleQuery">搜索</el-button>
                <el-button icon="el-icon-refresh" size="mini" @click="resetQuery">重置</el-button>
            </el-form-item>
        </el-form>

        <el-row :gutter="10" class="mb8">
            <el-col :span="1.5">
                <el-button type="primary" plain icon="el-icon-plus" size="mini" @click="handleAdd"
                    v-hasPermi="['monitor:policy:add']">新增</el-button>
            </el-col>
            <el-col :span="1.5">
                <el-button type="success" plain icon="el-icon-edit" size="mini" :disabled="single" @click="handleUpdate"
                    v-hasPermi="['monitor:policy:edit']">修改</el-button>
            </el-col>
            <el-col :span="1.5">
                <el-button type="danger" plain icon="el-icon-delete" size="mini" :disabled="multiple"
                    @click="handleDelete" v-hasPermi="['monitor:policy:remove']">删除</el-button>
            </el-col>
            <right-toolbar :showSearch.sync="showSearch" @queryTable="getList"></right-toolbar>
        </el-row>

        <el-table v-loading="loading" :data="policyList" @selection-change="handleSelectionChange">
            <el-table-column type="selection" width="55" align="center" />
            <el-table-column label="策略名称" align="center" prop="policyName" />
            <el-table-column label="是否默认" align="center" prop="isDefault">
                <template slot-scope="scope">
                    <el-tag v-if="scope.row.isDefault === '1'" type="success">是</el-tag>
                    <el-tag v-else type="info">否</el-tag>
                </template>
            </el-table-column>
            <el-table-column label="版本号" align="center" prop="version" />
            <el-table-column label="创建时间" align="center" prop="createTime" width="180">
                <template slot-scope="scope">
                    <span>{{ parseTime(scope.row.createTime) }}</span>
                </template>
            </el-table-column>
            <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
                <template slot-scope="scope">
                    <el-button size="mini" type="text" icon="el-icon-edit" @click="handleUpdate(scope.row)"
                        v-hasPermi="['monitor:policy:edit']">修改</el-button>
                    <el-button size="mini" type="text" icon="el-icon-delete" @click="handleDelete(scope.row)"
                        v-hasPermi="['monitor:policy:remove']">删除</el-button>
                </template>
            </el-table-column>
        </el-table>

        <pagination v-show="total > 0" :total="total" :page.sync="queryParams.pageNum" :limit.sync="queryParams.pageSize"
            @pagination="getList" />

        <!-- 添加或修改监控策略对话框 -->
        <el-dialog :title="title" :visible.sync="open" width="800px" append-to-body>
            <el-form ref="form" :model="form" :rules="rules" label-width="120px">
                <el-form-item label="策略名称" prop="policyName">
                    <el-input v-model="form.policyName" placeholder="请输入策略名称" />
                </el-form-item>
                <el-form-item label="策略版本" prop="version">
                    <el-input v-model="form.version" placeholder="请输入版本号，例如：1.0.0 或 20240115.01" />
                    <div class="help-block">建议使用语义化版本（如 1.0.1）或日期版本（如 2024.01.01）。客户端通过此版本号判断是否需要拉取新策略。</div>
                </el-form-item>

                <el-divider content-position="left">流量规则</el-divider>
                <el-form-item label="域名白名单">
                    <el-select v-model="rulesForm.traffic.whitelist" multiple filterable allow-create default-first-option
                        placeholder="请输入域名并按回车" style="width: 100%">
                        <el-option v-for="item in rulesForm.traffic.whitelist" :key="item" :label="item" :value="item" />
                    </el-select>
                    <div class="help-block">这些域名下的流量将不进行解密审计（如银行、政府网站）</div>
                </el-form-item>
                <el-form-item label="敏感词过滤">
                    <el-select v-model="rulesForm.traffic.keywords" multiple filterable allow-create default-first-option
                        placeholder="请输入敏感词并按回车" style="width: 100%">
                        <el-option v-for="item in rulesForm.traffic.keywords" :key="item" :label="item" :value="item" />
                    </el-select>
                </el-form-item>

                <el-divider content-position="left">截屏规则</el-divider>
                <el-row>
                    <el-col :span="12">
                        <el-form-item label="截屏间隔 (秒)">
                            <el-input-number v-model="rulesForm.screenshot.interval" :min="10" :max="3600" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="12">
                        <el-form-item label="开启 OCR">
                            <el-switch v-model="rulesForm.screenshot.ocr_enable" />
                        </el-form-item>
                    </el-col>
                </el-row>
                <el-form-item label="定向截图应用">
                    <el-select v-model="rulesForm.screenshot.apps" multiple filterable allow-create default-first-option
                        placeholder="请输入应用名称并按回车（如：WeChat）" style="width: 100%">
                        <el-option v-for="item in rulesForm.screenshot.apps" :key="item" :label="item" :value="item" />
                    </el-select>
                    <div class="help-block">配置后将仅对这些应用窗口进行截图审计；若不配置则对全屏进行周期性截图。</div>
                </el-form-item>
                <el-form-item label="OCR 敏感词" v-if="rulesForm.screenshot.ocr_enable">
                    <el-select v-model="rulesForm.screenshot.ocr_keywords" multiple filterable allow-create default-first-option
                        placeholder="请输入敏感词并按回车" style="width: 100%">
                        <el-option v-for="item in rulesForm.screenshot.ocr_keywords" :key="item" :label="item" :value="item" />
                    </el-select>
                    <div class="help-block">截图内容包含这些敏感词时将触发告警。</div>
                </el-form-item>

                <el-divider content-position="left">客户端设置</el-divider>
                <el-row>
                    <el-col :span="12">
                        <el-form-item label="心跳间隔 (秒)">
                            <el-input-number v-model="rulesForm.agent.heartbeatInterval" :min="10" :max="600" />
                        </el-form-item>
                    </el-col>
                    <el-col :span="12">
                        <el-form-item label="允许使用 USB">
                            <el-switch v-model="rulesForm.agent.allowUsb" />
                        </el-form-item>
                    </el-col>
                </el-row>
                <el-form-item label="进程黑名单">
                    <el-select v-model="rulesForm.agent.blockedProcesses" multiple filterable allow-create default-first-option
                        placeholder="请输入进程名并按回车（如：Terminal）" style="width: 100%">
                        <el-option v-for="item in rulesForm.agent.blockedProcesses" :key="item" :label="item" :value="item" />
                    </el-select>
                    <div class="help-block">配置后的进程在客户端将被禁止运行。</div>
                </el-form-item>

                <el-divider />
                <el-form-item label="是否默认" prop="isDefault">
                    <el-radio-group v-model="form.isDefault">
                        <el-radio label="1">是</el-radio>
                        <el-radio label="0">否</el-radio>
                    </el-radio-group>
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
    </div>
</template>

<script>
import { listPolicy, getPolicy, delPolicy, addPolicy, updatePolicy } from "@/api/monitor/policy";

export default {
    name: "Policy",
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
            // 监控策略表格数据
            policyList: [],
            // 弹出层标题
            title: "",
            // 是否显示弹出层
            open: false,
            // 查询参数
            queryParams: {
                pageNum: 1,
                pageSize: 10,
                policyName: null,
                isDefault: null,
            },
            // 表单参数
            form: {},
            // 可视化规则表单
            rulesForm: {
                traffic: {
                    whitelist: [],
                    keywords: []
                },
                screenshot: {
                    interval: 300,
                    ocr_enable: true,
                    apps: [],
                    ocr_keywords: []
                },
                agent: {
                    heartbeatInterval: 60,
                    allowUsb: true,
                    blockedProcesses: []
                }
            },
            // 表单校验
            rules: {
                policyName: [
                    { required: true, message: "策略名称不能为空", trigger: "blur" }
                ]
            }
        };
    },
    created() {
        this.getList();
    },
    methods: {
        /** 查询监控策略列表 */
        getList() {
            this.loading = true;
            listPolicy(this.queryParams).then(response => {
                this.policyList = response.rows;
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
                policyId: null,
                policyName: null,
                trafficRules: null,
                screenshotRules: null,
                agentSettings: null,
                isDefault: "0",
                remark: null
            };
            this.rulesForm = {
                traffic: {
                    whitelist: [],
                    keywords: []
                },
                screenshot: {
                    interval: 300,
                    ocr_enable: true,
                    apps: [],
                    ocr_keywords: []
                },
                agent: {
                    heartbeatInterval: 60,
                    allowUsb: true,
                    blockedProcesses: []
                }
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
            this.ids = selection.map(item => item.policyId)
            this.single = selection.length !== 1
            this.multiple = !selection.length
        },
        /** 新增按钮操作 */
        handleAdd() {
            this.reset();
            this.open = true;
            this.title = "添加监控策略";
        },
        /** 修改按钮操作 */
        handleUpdate(row) {
            this.reset();
            const policyId = row.policyId || this.ids
            getPolicy(policyId).then(response => {
                this.form = response.data;
                this.parseRules();
                this.open = true;
                this.title = "修改监控策略";
            });
        },
        /** 解析 JSON 规则到可视化表单 */
        parseRules() {
            try {
                if (this.form.trafficRules) {
                    const traffic = JSON.parse(this.form.trafficRules);
                    this.rulesForm.traffic.whitelist = traffic.whitelist || [];
                    this.rulesForm.traffic.keywords = traffic.keywords || [];
                }
                if (this.form.screenshotRules) {
                    const screenshot = JSON.parse(this.form.screenshotRules);
                    this.rulesForm.screenshot.interval = screenshot.interval || 300;
                    this.rulesForm.screenshot.ocr_enable = screenshot.ocr_enable !== false;
                    this.rulesForm.screenshot.apps = screenshot.apps || [];
                    this.rulesForm.screenshot.ocr_keywords = screenshot.ocr_keywords || [];
                }
                if (this.form.agentSettings) {
                    const agent = JSON.parse(this.form.agentSettings);
                    this.rulesForm.agent.heartbeatInterval = agent.heartbeatInterval || 60;
                    this.rulesForm.agent.allowUsb = agent.allowUsb !== false;
                    this.rulesForm.agent.blockedProcesses = agent.blockedProcesses || [];
                }
            } catch (e) {
                console.error("解析策略规则失败", e);
            }
        },
        /** 提交按钮 */
        submitForm() {
            this.$refs["form"].validate(valid => {
                if (valid) {
                    // 将可视化表单数据转换回 JSON 字符串
                    this.form.trafficRules = JSON.stringify(this.rulesForm.traffic);
                    this.form.screenshotRules = JSON.stringify(this.rulesForm.screenshot);
                    this.form.agentSettings = JSON.stringify(this.rulesForm.agent);

                    if (this.form.policyId != null) {
                        updatePolicy(this.form).then(response => {
                            this.$modal.msgSuccess("修改成功");
                            this.open = false;
                            this.getList();
                        });
                    } else {
                        addPolicy(this.form).then(response => {
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
            const policyIds = row.policyId || this.ids;
            this.$modal.confirm('是否确认删除监控策略编号为"' + policyIds + '"的数据项？').then(function () {
                return delPolicy(policyIds);
            }).then(() => {
                this.getList();
                this.$modal.msgSuccess("删除成功");
            }).catch(() => { });
        }
    }
};
</script>

<style scoped>
.help-block {
    color: #909399;
    font-size: 12px;
    line-height: 1.5;
    margin-top: 5px;
}

.el-divider--horizontal {
    margin: 24px 0 16px 0;
}

.el-divider__text {
    font-weight: bold;
    color: #303133;
}
</style>
