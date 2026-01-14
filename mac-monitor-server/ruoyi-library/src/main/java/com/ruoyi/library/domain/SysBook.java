package com.ruoyi.library.domain;

import java.util.ArrayList;
import java.util.List;
import java.math.BigDecimal;
import org.apache.commons.lang3.builder.ToStringBuilder;
import org.apache.commons.lang3.builder.ToStringStyle;
import com.ruoyi.common.annotation.Excel;
import com.ruoyi.common.core.domain.BaseEntity;
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import javax.validation.constraints.*;

/**
 * 图书管理对象 sys_book
 *
 * @author ruoyi
 */
@ApiModel("图书管理实体")
public class SysBook extends BaseEntity
{
    private static final long serialVersionUID = 1L;

    /** 图书ID */
    @ApiModelProperty("图书ID")
    private Long bookId;

    /** 父图书ID */
    @ApiModelProperty("父图书ID")
    @NotNull(message = "上级图书不能为空")
    private Long parentId;

    /** 祖级列表 */
    @ApiModelProperty("祖级列表")
    private String ancestors;

    /** 图书名称 */
    @ApiModelProperty("图书名称")
    @Excel(name = "图书名称")
    @NotBlank(message = "图书名称不能为空")
    @Size(min = 0, max = 100, message = "图书名称长度不能超过100个字符")
    private String bookName;

    /** 显示顺序 */
    @ApiModelProperty("显示顺序")
    @NotNull(message = "显示顺序不能为空")
    private Integer orderNum;

    /** 作者 */
    @ApiModelProperty("作者")
    @Excel(name = "作者")
    @Size(min = 0, max = 64, message = "作者名称长度不能超过64个字符")
    private String author;

    /** 价格 */
    @ApiModelProperty("价格")
    @Excel(name = "价格")
    @DecimalMin(value = "0", message = "价格不能小于0")
    private BigDecimal price;

    /** 状态（0正常 1停用） */
    @ApiModelProperty("状态（0正常 1停用）")
    @Excel(name = "状态", readConverterExp = "0=正常,1=停用")
    private String status;

    /** 图书分类 */
    @ApiModelProperty("图书分类")
    @Excel(name = "图书分类", dictType = "sys_book_category")
    private String category;

    /** 图书封面 */
    @ApiModelProperty("图书封面")
    @Excel(name = "图书封面")
    private String coverUrl;

    /** 子图书列表 */
    private List<SysBook> children = new ArrayList<SysBook>();

    /** 图书章节信息 */
    private List<SysBookChapter> sysBookChapterList;

    public void setBookId(Long bookId) {
        this.bookId = bookId;
    }
    public Long getBookId() {
        return bookId;
    }

    public void setParentId(Long parentId) {
        this.parentId = parentId;
    }
    public Long getParentId() {
        return parentId;
    }

    public void setAncestors(String ancestors) {
        this.ancestors = ancestors;
    }
    public String getAncestors() {
        return ancestors;
    }

    public void setBookName(String bookName) {
        this.bookName = bookName;
    }
    public String getBookName() {
        return bookName;
    }

    public void setOrderNum(Integer orderNum) {
        this.orderNum = orderNum;
    }
    public Integer getOrderNum() {
        return orderNum;
    }

    public void setAuthor(String author) {
        this.author = author;
    }
    public String getAuthor() {
        return author;
    }

    public void setPrice(BigDecimal price) {
        this.price = price;
    }
    public BigDecimal getPrice() {
        return price;
    }

    public void setStatus(String status) {
        this.status = status;
    }
    public String getStatus() {
        return status;
    }

    public void setCategory(String category) {
        this.category = category;
    }
    public String getCategory() {
        return category;
    }

    public void setCoverUrl(String coverUrl) {
        this.coverUrl = coverUrl;
    }
    public String getCoverUrl() {
        return coverUrl;
    }

    public List<SysBook> getChildren() {
        return children;
    }
    public void setChildren(List<SysBook> children) {
        this.children = children;
    }

    public List<SysBookChapter> getSysBookChapterList()
    {
        return sysBookChapterList;
    }

    public void setSysBookChapterList(List<SysBookChapter> sysBookChapterList)
    {
        this.sysBookChapterList = sysBookChapterList;
    }

    @Override
    public String toString() {
        return new ToStringBuilder(this,ToStringStyle.MULTI_LINE_STYLE)
            .append("bookId", getBookId())
            .append("parentId", getParentId())
            .append("ancestors", getAncestors())
            .append("bookName", getBookName())
            .append("orderNum", getOrderNum())
            .append("author", getAuthor())
            .append("price", getPrice())
            .append("status", getStatus())
            .append("createBy", getCreateBy())
            .append("createTime", getCreateTime())
            .append("updateBy", getUpdateBy())
            .append("updateTime", getUpdateTime())
            .append("remark", getRemark())
            .toString();
    }
}
