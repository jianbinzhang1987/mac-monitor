<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryForm" size="small" :inline="true" v-show="showSearch">
      <el-form-item label="图书名称" prop="bookName">
        <el-input
          v-model="queryParams.bookName"
          placeholder="请输入图书名称"
          clearable
          @keyup.enter.native="handleQuery"
        />
      </el-form-item>
      <el-form-item label="作者" prop="author">
        <el-input
          v-model="queryParams.author"
          placeholder="请输入作者"
          clearable
          @keyup.enter.native="handleQuery"
        />
      </el-form-item>
      <el-form-item label="图书分类" prop="category">
        <el-select v-model="queryParams.category" placeholder="请选择分类" clearable>
          <el-option
            v-for="dict in dict.type.sys_book_category"
            :key="dict.value"
            :label="dict.label"
            :value="dict.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="状态" prop="status">
        <el-select v-model="queryParams.status" placeholder="图书状态" clearable>
          <el-option
            v-for="dict in dict.type.sys_normal_disable"
            :key="dict.value"
            :label="dict.label"
            :value="dict.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="创建时间">
        <el-date-picker
          v-model="dateRange"
          style="width: 240px"
          value-format="yyyy-MM-dd"
          type="daterange"
          range-separator="-"
          start-placeholder="开始日期"
          end-placeholder="结束日期"
        ></el-date-picker>
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
          v-hasPermi="['library:book:add']"
        >新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="info"
          plain
          icon="el-icon-sort"
          size="mini"
          @click="toggleExpandAll"
        >展开/折叠</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="info"
          plain
          icon="el-icon-upload2"
          size="mini"
          @click="handleImport"
          v-hasPermi="['library:book:import']"
        >导入</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
          type="warning"
          plain
          icon="el-icon-download"
          size="mini"
          @click="handleExport"
          v-hasPermi="['library:book:export']"
        >导出</el-button>
      </el-col>
      <right-toolbar :showSearch.sync="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table
      v-if="refreshTable"
      v-loading="loading"
      :data="bookList"
      row-key="bookId"
      :default-expand-all="isExpandAll"
      :tree-props="{children: 'children', hasChildren: 'hasChildren'}"
    >
      <el-table-column prop="bookName" label="图书名称" width="260"></el-table-column>
      <el-table-column label="分类" align="center" prop="category" width="100">
        <template slot-scope="scope">
          <dict-tag :options="dict.type.sys_book_category" :value="scope.row.category"/>
        </template>
      </el-table-column>
      <el-table-column label="封面" align="center" prop="coverUrl" width="100">
        <template slot-scope="scope">
          <image-preview :src="scope.row.coverUrl" :width="50" :height="50"/>
        </template>
      </el-table-column>
      <el-table-column prop="author" label="作者" width="150"></el-table-column>
      <el-table-column prop="price" label="价格" width="100"></el-table-column>
      <el-table-column prop="orderNum" label="排序" width="80"></el-table-column>
      <el-table-column prop="status" label="状态" width="100">
        <template slot-scope="scope">
          <dict-tag :options="dict.type.sys_normal_disable" :value="scope.row.status"/>
        </template>
      </el-table-column>
      <el-table-column label="创建时间" align="center" prop="createTime" width="180">
        <template slot-scope="scope">
          <span>{{ parseTime(scope.row.createTime) }}</span>
        </template>
      </el-table-column>
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template slot-scope="scope">
          <el-button
            size="mini"
            type="text"
            icon="el-icon-edit"
            @click="handleUpdate(scope.row)"
            v-hasPermi="['library:book:edit']"
          >修改</el-button>
          <el-button
            size="mini"
            type="text"
            icon="el-icon-plus"
            @click="handleAdd(scope.row)"
            v-hasPermi="['library:book:add']"
          >新增</el-button>
          <el-button
            v-if="scope.row.parentId != 0"
            size="mini"
            type="text"
            icon="el-icon-delete"
            @click="handleDelete(scope.row)"
            v-hasPermi="['library:book:remove']"
          >删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 添加或修改图书对话框 -->
    <el-dialog :title="title" :visible.sync="open" width="600px" append-to-body>
      <el-form ref="form" :model="form" :rules="rules" label-width="80px">
        <el-row>
          <el-col :span="24">
            <el-form-item label="上级图书" prop="parentId">
              <treeselect v-model="form.parentId" :options="bookOptions" :normalizer="normalizer" placeholder="选择上级图书" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="图书名称" prop="bookName">
              <el-input v-model="form.bookName" placeholder="请输入图书名称" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="显示排序" prop="orderNum">
              <el-input-number v-model="form.orderNum" controls-position="right" :min="0" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="作者" prop="author">
              <el-input v-model="form.author" placeholder="请输入作者" maxlength="64" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="价格" prop="price">
              <el-input-number v-model="form.price" :precision="2" :step="0.1" :min="0" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="图书分类" prop="category">
              <el-select v-model="form.category" placeholder="请选择分类">
                <el-option
                  v-for="dict in dict.type.sys_book_category"
                  :key="dict.value"
                  :label="dict.label"
                  :value="dict.value"
                ></el-option>
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="图书状态">
              <el-radio-group v-model="form.status">
                <el-radio
                  v-for="dict in dict.type.sys_normal_disable"
                  :key="dict.value"
                  :label="dict.value"
                >{{dict.label}}</el-radio>
              </el-radio-group>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="24">
            <el-form-item label="图书封面">
              <image-upload v-model="form.coverUrl"/>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="24">
            <el-form-item label="备注" prop="remark">
              <el-input v-model="form.remark" type="textarea" placeholder="请输入内容"></el-input>
            </el-form-item>
          </el-col>
        </el-row>
        <el-divider content-position="center">图书章节信息</el-divider>
        <el-row :gutter="10" class="mb8">
          <el-col :span="1.5">
            <el-button type="primary" icon="el-icon-plus" size="mini" @click="handleAddSysBookChapter">添加</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button type="danger" icon="el-icon-delete" size="mini" @click="handleDeleteSysBookChapter">删除</el-button>
          </el-col>
        </el-row>
        <el-table :data="sysBookChapterList" :row-class-name="rowSysBookChapterIndex" @selection-change="handleSysBookChapterSelectionChange" ref="sysBookChapter">
          <el-table-column type="selection" width="50" align="center" />
          <el-table-column label="序号" align="center" prop="index" width="50"/>
          <el-table-column label="章节标题" prop="chapterTitle" width="150">
            <template slot-scope="scope">
              <el-input v-model="scope.row.chapterTitle" placeholder="请输入章节标题" />
            </template>
          </el-table-column>
          <el-table-column label="排序" prop="orderNum" width="120">
            <template slot-scope="scope">
              <el-input-number v-model="scope.row.orderNum" controls-position="right" :min="0" />
            </template>
          </el-table-column>
          <el-table-column label="内容摘要" prop="contentSummary" width="150">
            <template slot-scope="scope">
              <el-input v-model="scope.row.contentSummary" placeholder="请输入内容摘要" />
            </template>
          </el-table-column>
        </el-table>
      </el-form>
      <div slot="footer" class="dialog-footer">
        <el-button type="primary" @click="submitForm">确 定</el-button>
        <el-button @click="cancel">取 消</el-button>
      </div>
    </el-dialog>

    <!-- 图书导入对话框 -->
    <el-dialog :title="upload.title" :visible.sync="upload.open" width="400px" append-to-body>
      <el-upload
        ref="upload"
        :limit="1"
        accept=".xlsx, .xls"
        :headers="upload.headers"
        :action="upload.url + '?updateSupport=' + upload.updateSupport"
        :disabled="upload.isUploading"
        :on-progress="handleFileUploadProgress"
        :on-success="handleFileSuccess"
        :auto-upload="false"
        drag
      >
        <i class="el-icon-upload"></i>
        <div class="el-upload__text">将文件拖到此处，或<em>点击上传</em></div>
        <div class="el-upload__tip text-center" slot="tip">
          <div class="el-upload__tip" slot="tip">
            <el-checkbox v-model="upload.updateSupport" /> 是否更新已经存在的图书数据
          </div>
          <span>仅允许导入xls、xlsx格式文件。</span>
          <el-link type="primary" :underline="false" style="font-size:12px;vertical-align: baseline;" @click="importTemplate">下载模板</el-link>
        </div>
      </el-upload>
      <div slot="footer" class="dialog-footer">
        <el-button type="primary" @click="submitFileForm">确 定</el-button>
        <el-button @click="upload.open = false">取 消</el-button>
      </div>
    </el-dialog>
  </div>
</template>

<script>
import { listBook, getBook, delBook, addBook, updateBook } from "@/api/library/book";
import Treeselect from "@riophae/vue-treeselect";
import "@riophae/vue-treeselect/dist/vue-treeselect.css";
import { getToken } from "@/utils/auth";

export default {
  name: "Book",
  dicts: ['sys_normal_disable', 'sys_book_category'],
  components: { Treeselect },
  data() {
    return {
      // 遮罩层
      loading: true,
      // 显示搜索条件
      showSearch: true,
      // 图书表格数据
      bookList: [],
      // 图书树选项
      bookOptions: [],
      // 弹出层标题
      title: "",
      // 是否显示弹出层
      open: false,
      // 是否展开，默认全部展开
      isExpandAll: true,
      // 重新渲染表格状态
      refreshTable: true,
      // 日期范围
      dateRange: [],
      // 图书章节表格数据
      sysBookChapterList: [],
      // 子表选中数据
      checkedSysBookChapter: [],
      // 查询参数
      queryParams: {
        bookName: undefined,
        author: undefined,
        status: undefined,
        category: undefined
      },
      // 表单参数
      form: {},
      // 表单校验
      rules: {
        parentId: [
          { required: true, message: "上级图书不能为空", trigger: "blur" }
        ],
        bookName: [
          { required: true, message: "图书名称不能为空", trigger: "blur" },
          { min: 2, max: 100, message: "图书名称长度必须在 2 到 100 个字符之间", trigger: "blur" }
        ],
        orderNum: [
          { required: true, message: "显示排序不能为空", trigger: "blur" }
        ],
        author: [
          { max: 64, message: "作者名称长度不能超过 64 个字符", trigger: "blur" }
        ],
        price: [
          { type: 'number', min: 0, message: "价格不能小于 0", trigger: "blur" }
        ]
      },
      // 用户导入参数
      upload: {
        // 是否显示弹出层（用户导入）
        open: false,
        // 弹出层标题（用户导入）
        title: "",
        // 是否禁用上传
        isUploading: false,
        // 是否更新已经存在的数据
        updateSupport: 0,
        // 设置上传的请求头部
        headers: { Authorization: "Bearer " + getToken() },
        // 上传的地址
        url: process.env.VUE_APP_BASE_API + "/library/book/importData"
      }
    };
  },
  created() {
    this.getList();
  },
  methods: {
    /** 查询图书列表 */
    getList() {
      this.loading = true;
      listBook(this.addDateRange(this.queryParams, this.dateRange)).then(response => {
        this.bookList = this.handleTree(response.data, "bookId");
        this.loading = false;
      });
    },
    /** 转换图书数据结构 */
    normalizer(node) {
      if (node.children && !node.children.length) {
        delete node.children;
      }
      return {
        id: node.bookId,
        label: node.bookName,
        children: node.children
      };
    },
    /** 查询图书下拉树结构 */
    getTreeselect() {
      listBook().then(response => {
        this.bookOptions = [];
        const data = { bookId: 0, bookName: '顶级节点', children: [] };
        data.children = this.handleTree(response.data, "bookId");
        this.bookOptions.push(data);
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
        bookId: undefined,
        parentId: undefined,
        bookName: undefined,
        orderNum: 0,
        author: undefined,
        price: 0,
        status: "0",
        category: undefined,
        coverUrl: undefined,
        remark: undefined,
        sysBookChapterList: []
      };
      this.sysBookChapterList = [];
      this.resetForm("form");
    },
    /** 搜索按钮操作 */
    handleQuery() {
      this.getList();
    },
    /** 重置按钮操作 */
    resetQuery() {
      this.dateRange = [];
      this.resetForm("queryForm");
      this.handleQuery();
    },
    /** 新增按钮操作 */
    handleAdd(row) {
      this.reset();
      this.getTreeselect();
      if (row != undefined) {
        this.form.parentId = row.bookId;
      }
      this.open = true;
      this.title = "添加图书";
    },
    /** 展开/折叠操作 */
    toggleExpandAll() {
      this.refreshTable = false;
      this.isExpandAll = !this.isExpandAll;
      this.$nextTick(() => {
        this.refreshTable = true;
      });
    },
    /** 修改按钮操作 */
    handleUpdate(row) {
      this.reset();
      this.getTreeselect();
      if (row != undefined) {
        this.form.parentId = row.bookId;
      }
      getBook(row.bookId).then(response => {
        this.form = response.data;
        this.sysBookChapterList = response.data.sysBookChapterList;
        this.open = true;
        this.title = "修改图书";
      });
    },
    /** 提交按钮 */
    submitForm: function() {
      this.$refs["form"].validate(valid => {
        if (valid) {
          this.form.sysBookChapterList = this.sysBookChapterList;
          if (this.form.bookId != undefined) {
            updateBook(this.form).then(response => {
              this.$modal.msgSuccess("修改成功");
              this.open = false;
              this.getList();
            });
          } else {
            addBook(this.form).then(response => {
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
      this.$modal.confirm('是否确认删除图书名称为"' + row.bookName + '"的数据项？').then(function() {
        return delBook(row.bookId);
      }).then(() => {
        this.getList();
        this.$modal.msgSuccess("删除成功");
      }).catch(() => {});
    },
    /** 导出按钮操作 */
    handleExport() {
      this.download('library/book/export', {
        ...this.queryParams
      }, `book_${new Date().getTime()}.xlsx`)
    },
    /** 图书章节序号 */
    rowSysBookChapterIndex({ row, rowIndex }) {
      row.index = rowIndex + 1;
    },
    /** 图书章节添加按钮操作 */
    handleAddSysBookChapter() {
      let obj = {};
      obj.chapterTitle = "";
      obj.orderNum = "";
      obj.contentSummary = "";
      this.sysBookChapterList.push(obj);
    },
    /** 图书章节删除按钮操作 */
    handleDeleteSysBookChapter() {
      if (this.checkedSysBookChapter.length == 0) {
        this.$modal.msgError("请先选择要删除的图书章节数据");
      } else {
        const sysBookChapterList = this.sysBookChapterList;
        const checkedSysBookChapter = this.checkedSysBookChapter;
        this.sysBookChapterList = sysBookChapterList.filter(function(item) {
          return checkedSysBookChapter.indexOf(item.index) == -1
        });
      }
    },
    /** 复选框选中数据 */
    handleSysBookChapterSelectionChange(selection) {
      this.checkedSysBookChapter = selection.map(item => item.index)
    },
    /** 导入按钮操作 */
    handleImport() {
      this.upload.title = "图书导入";
      this.upload.open = true;
    },
    /** 下载模板操作 */
    importTemplate() {
      this.download('library/book/importTemplate', {
      }, `book_template_${new Date().getTime()}.xlsx`)
    },
    // 文件上传中处理
    handleFileUploadProgress(event, file, fileList) {
      this.upload.isUploading = true;
    },
    // 文件上传成功处理
    handleFileSuccess(response, file, fileList) {
      this.upload.open = false;
      this.upload.isUploading = false;
      this.$refs.upload.clearFiles();
      this.$alert("<div style='overflow: auto;overflow-x: hidden;max-height: 70vh;padding: 10px 20px 0;'>" + response.msg + "</div>", "导入结果", { dangerouslyUseHTMLString: true });
      this.getList();
    },
    // 提交上传文件
    submitFileForm() {
      this.$refs.upload.submit();
    }
  }
};
</script>
