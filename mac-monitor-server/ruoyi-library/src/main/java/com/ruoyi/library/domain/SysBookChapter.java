package com.ruoyi.library.domain;

import org.apache.commons.lang3.builder.ToStringBuilder;
import org.apache.commons.lang3.builder.ToStringStyle;
import com.ruoyi.common.annotation.Excel;
import com.ruoyi.common.core.domain.BaseEntity;
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import javax.validation.constraints.*;

/**
 * 图书章节对象 sys_book_chapter
 *
 * @author ruoyi
 */
@ApiModel("图书章节实体")
public class SysBookChapter extends BaseEntity
{
    private static final long serialVersionUID = 1L;

    /** 章节ID */
    @ApiModelProperty("章节ID")
    private Long chapterId;

    /** 图书ID */
    @ApiModelProperty("图书ID")
    private Long bookId;

    /** 章节标题 */
    @ApiModelProperty("章节标题")
    @Excel(name = "章节标题")
    @NotBlank(message = "章节标题不能为空")
    @Size(min = 0, max = 255, message = "章节标题长度不能超过255个字符")
    private String chapterTitle;

    /** 排序 */
    @ApiModelProperty("排序")
    @Excel(name = "排序")
    @NotNull(message = "排序不能为空")
    private Integer orderNum;

    /** 内容摘要 */
    @ApiModelProperty("内容摘要")
    @Excel(name = "内容摘要")
    private String contentSummary;

    public void setChapterId(Long chapterId) {
        this.chapterId = chapterId;
    }
    public Long getChapterId() {
        return chapterId;
    }

    public void setBookId(Long bookId) {
        this.bookId = bookId;
    }
    public Long getBookId() {
        return bookId;
    }

    public void setChapterTitle(String chapterTitle) {
        this.chapterTitle = chapterTitle;
    }
    public String getChapterTitle() {
        return chapterTitle;
    }

    public void setOrderNum(Integer orderNum) {
        this.orderNum = orderNum;
    }
    public Integer getOrderNum() {
        return orderNum;
    }

    public void setContentSummary(String contentSummary) {
        this.contentSummary = contentSummary;
    }
    public String getContentSummary() {
        return contentSummary;
    }

    @Override
    public String toString() {
        return new ToStringBuilder(this,ToStringStyle.MULTI_LINE_STYLE)
            .append("chapterId", getChapterId())
            .append("bookId", getBookId())
            .append("chapterTitle", getChapterTitle())
            .append("orderNum", getOrderNum())
            .append("contentSummary", getContentSummary())
            .toString();
    }
}
