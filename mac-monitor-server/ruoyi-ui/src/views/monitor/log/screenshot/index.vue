<template>
    <div class="app-container">
        <el-form :model="queryParams" ref="queryForm" size="small" :inline="true" v-show="showSearch"
            label-width="68px">
            <el-form-item label="设备号" prop="serialNumber">
                <el-input v-model="queryParams.serialNumber" placeholder="请输入设备序列号" clearable
                    @keyup.enter.native="handleQuery" />
            </el-form-item>
            <el-form-item label="风险等级" prop="riskLevel">
                <el-select v-model="queryParams.riskLevel" placeholder="请选择风险等级" clearable>
                    <el-option label="低" value="0" />
                    <el-option label="中" value="1" />
                    <el-option label="高" value="2" />
                </el-select>
            </el-form-item>
            <el-form-item label="截图时间">
                <el-date-picker v-model="dateRange" style="width: 240px" value-format="yyyy-MM-dd HH:mm:ss"
                    type="daterange" range-separator="-" start-placeholder="开始日期"
                    end-placeholder="结束日期"></el-date-picker>
            </el-form-item>
            <el-form-item>
                <el-button type="primary" icon="el-icon-search" size="mini" @click="handleQuery">搜索</el-button>
                <el-button icon="el-icon-refresh" size="mini" @click="resetQuery">重置</el-button>
            </el-form-item>
        </el-form>

        <el-row :gutter="10" class="mb8">
            <el-col :span="1.5">
                <el-button type="warning" plain icon="el-icon-download" size="mini" @click="handleExport"
                    v-hasPermi="['monitor:log:screenshot:export']">导出</el-button>
            </el-col>
            <el-col :span="1.5">
                <el-button type="danger" plain icon="el-icon-delete" size="mini" :disabled="multiple"
                    @click="handleDelete" v-hasPermi="['monitor:log:screenshot:remove']">删除</el-button>
            </el-col>
            <right-toolbar :showSearch.sync="showSearch" @queryTable="getList"></right-toolbar>
        </el-row>

        <el-table v-loading="loading" :data="screenshotList" @selection-change="handleSelectionChange" border>
            <el-table-column type="selection" width="55" align="center" />
            <el-table-column label="序列号" align="center" prop="serialNumber" width="150" />
            <el-table-column label="图片预览" align="center" width="120">
                <template slot-scope="scope">
                    <el-image
                        style="width: 80px; height: 45px; border-radius: 4px; border: 1px solid #ebeef5; cursor: pointer;"
                        :src="scope.row.filePath"
                        :preview-src-list="[scope.row.filePath]"
                        fit="cover"
                    >
                        <div slot="error" class="image-slot">
                            <i class="el-icon-picture-outline"></i>
                        </div>
                    </el-image>
                </template>
            </el-table-column>
            <el-table-column label="风险等级" align="center" prop="riskLevel" width="100">
                <template slot-scope="scope">
                    <el-tag v-if="scope.row.riskLevel == 0" type="success" effect="plain">低风险</el-tag>
                    <el-tag v-else-if="scope.row.riskLevel == 1" type="warning" effect="dark">中风险</el-tag>
                    <el-tag v-else type="danger" effect="dark">高风险</el-tag>
                </template>
            </el-table-column>
            <el-table-column label="OCR识别内容" align="left" prop="ocrText">
                <template slot-scope="scope">
                    <div class="ocr-text-container">
                        {{ scope.row.ocrText }}
                    </div>
                </template>
            </el-table-column>
            <el-table-column label="截图时间" align="center" prop="captureTime" width="160">
                <template slot-scope="scope">
                    <span>{{ parseTime(scope.row.captureTime) }}</span>
                </template>
            </el-table-column>
            <el-table-column label="操作" align="center" class-name="small-padding fixed-width" width="150">
                <template slot-scope="scope">
                    <el-button size="mini" type="text" icon="el-icon-plus" @click="handleAddKeyword(scope.row)"
                        v-hasPermi="['monitor:policy:edit']">加入策略</el-button>
                    <el-button size="mini" type="text" icon="el-icon-delete" @click="handleDelete(scope.row)"
                        v-hasPermi="['monitor:log:screenshot:remove']">删除</el-button>
                </template>
            </el-table-column>
        </el-table>

        <!-- 提取敏感词对话框 -->
        <el-dialog :title="keywordTitle" :visible.sync="keywordOpen" width="500px" append-to-body>
            <el-form ref="keywordForm" :model="keywordForm" label-width="80px">
                <el-form-item label="设备序列号">
                    <el-input v-model="keywordForm.serialNumber" readonly />
                </el-form-item>
                <el-form-item label="OCR内容">
                    <el-input type="textarea" v-model="keywordForm.ocrText" readonly :rows="4" />
                </el-form-item>
                <el-form-item label="提取词" prop="keyword">
                    <el-input v-model="keywordForm.keyword" placeholder="请输入要加入策略的敏感词" />
                </el-form-item>
            </el-form>
            <div slot="footer" class="dialog-footer">
                <el-button type="primary" @click="submitKeyword">确认加入</el-button>
                <el-button @click="keywordOpen = false">取消</el-button>
            </div>
        </el-dialog>

        <pagination v-show="total > 0" :total="total" :page.sync="queryParams.pageNum" :limit.sync="queryParams.pageSize"
            @pagination="getList" />
    </div>
</template>

<script>
import { listScreenshot, getScreenshot, delScreenshot } from "@/api/monitor/screenshot";
import { listPolicy, updatePolicy } from "@/api/monitor/policy";

export default {
    name: "ScreenshotLog",
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
            // 截图日志表格数据
            screenshotList: [],
            // 日期范围
            dateRange: [],
            // 提取敏感词相关
            keywordOpen: false,
            keywordForm: {
                ocrText: '',
                keyword: ''
            },
            // 查询参数
            queryParams: {
                pageNum: 1,
                pageSize: 10,
                serialNumber: null,
                riskLevel: null,
            }
        };
    },
    created() {
        this.getList();
    },
    methods: {
        /** 查询截图日志列表 */
        getList() {
            this.loading = true;
            listScreenshot(this.addDateRange(this.queryParams, this.dateRange)).then(response => {
                this.screenshotList = response.rows;
                this.total = response.total;
                this.loading = false;
            });
        },
        /** 提取敏感词按钮操作 */
        handleAddKeyword(row) {
            this.keywordForm = {
                serialNumber: row.serialNumber,
                ocrText: row.ocrText,
                keyword: ''
            };
            this.keywordTitle = "提取敏感词 - 设备: " + row.serialNumber;
            this.keywordOpen = true;
        },
        /** 提交提取的敏感词 */
        submitKeyword() {
            if (!this.keywordForm.keyword) {
                this.$modal.msgError("请输入要加入的敏感词");
                return;
            }

            this.loading = true;
            // 1. 根据序列号查找设备，确认关联策略
            import("@/api/monitor/device").then(deviceApi => {
                deviceApi.listDevice({ serialNumber: this.keywordForm.serialNumber }).then(response => {
                    let policyPromise;
                    if (response.rows && response.rows.length > 0 && response.rows[0].policyId) {
                        // 找到设备绑定的策略
                        policyPromise = getPolicy(response.rows[0].policyId);
                    } else {
                        // 回退到默认策略
                        policyPromise = listPolicy({ isDefault: '1' }).then(res => {
                            return res.rows && res.rows.length > 0 ? { data: res.rows[0] } : Promise.reject("未找到策略");
                        });
                    }

                    policyPromise.then(res => {
                        const policy = res.data;
                        let screenshotRules = {};
                        try {
                            screenshotRules = JSON.parse(policy.screenshotRules || '{}');
                        } catch (e) {
                            screenshotRules = {};
                        }

                        if (!screenshotRules.ocr_keywords) {
                            screenshotRules.ocr_keywords = [];
                        }

                        if (!screenshotRules.ocr_keywords.includes(this.keywordForm.keyword)) {
                            screenshotRules.ocr_keywords.push(this.keywordForm.keyword);
                            policy.screenshotRules = JSON.stringify(screenshotRules);

                            updatePolicy(policy).then(updateRes => {
                                this.$modal.msgSuccess(`成功加入策略 [${policy.policyName}]`);
                                this.keywordOpen = false;
                                this.loading = false;
                            });
                        } else {
                            this.$modal.msgWarning("该关键词已在策略中");
                            this.keywordOpen = false;
                            this.loading = false;
                        }
                    }).catch(err => {
                        this.$modal.msgError("获取策略失败: " + err);
                        this.loading = false;
                    });
                });
            });
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
            this.$modal.confirm('是否确认删除截图日志编号为"' + logIds + '"的数据项？').then(function () {
                return delScreenshot(logIds);
            }).then(() => {
                this.getList();
                this.$modal.msgSuccess("删除成功");
            }).catch(() => { });
        },
        /** 导出按钮操作 */
        handleExport() {
            this.download('monitor/log/screenshot/export', {
                ...this.queryParams
            }, `screenshot_${new Date().getTime()}.xlsx`)
        }
    }
};
</script>

<style scoped>
.ocr-text-container {
    max-height: 100px;
    overflow-y: auto;
    font-size: 13px;
    color: #606266;
    line-height: 1.6;
    text-align: left;
    white-space: pre-wrap;
    word-break: break-all;
    padding: 8px;
    background-color: #f8f9fa;
    border-radius: 4px;
    border: 1px solid #ebeef5;
}

.image-slot {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    background: #f5f7fa;
    color: #909399;
    font-size: 20px;
}
</style>
