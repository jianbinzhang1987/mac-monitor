package com.ruoyi.asset.domain;

import java.util.Date;
import java.math.BigDecimal;
import org.apache.commons.lang3.builder.ToStringBuilder;
import org.apache.commons.lang3.builder.ToStringStyle;
import com.ruoyi.common.annotation.Excel;
import com.ruoyi.common.core.domain.BaseEntity;
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

/**
 * 资产信息对象 biz_asset
 *
 * @author ruoyi
 */
@ApiModel("资产信息实体")
public class BizAsset extends BaseEntity
{
    private static final long serialVersionUID = 1L;

    /** 资产ID */
    @ApiModelProperty("资产ID")
    private Long assetId;

    /** 资产编号 */
    @ApiModelProperty("资产编号")
    @Excel(name = "资产编号")
    private String assetCode;

    /** 资产名称 */
    @ApiModelProperty("资产名称")
    @Excel(name = "资产名称")
    private String assetName;

    /** 资产类型 */
    @ApiModelProperty("资产类型")
    @Excel(name = "资产类型", dictType = "biz_asset_type")
    private String assetType;

    /** 状态 */
    @ApiModelProperty("状态")
    @Excel(name = "状态", dictType = "biz_asset_status")
    private String status;

    /** 购入日期 */
    @ApiModelProperty("购入日期")
    @Excel(name = "购入日期", width = 30, dateFormat = "yyyy-MM-dd")
    private Date purchaseDate;

    /** 单价 */
    @ApiModelProperty("单价")
    @Excel(name = "单价")
    private BigDecimal price;

    /** 存放地点 */
    @ApiModelProperty("存放地点")
    @Excel(name = "存放地点")
    private String location;

    /** 管理人 */
    @ApiModelProperty("管理人")
    @Excel(name = "管理人")
    private String manager;

    /** 删除标志 */
    private String delFlag;

    public void setAssetId(Long assetId)
    {
        this.assetId = assetId;
    }

    public Long getAssetId()
    {
        return assetId;
    }
    public void setAssetCode(String assetCode)
    {
        this.assetCode = assetCode;
    }

    public String getAssetCode()
    {
        return assetCode;
    }
    public void setAssetName(String assetName)
    {
        this.assetName = assetName;
    }

    public String getAssetName()
    {
        return assetName;
    }
    public void setAssetType(String assetType)
    {
        this.assetType = assetType;
    }

    public String getAssetType()
    {
        return assetType;
    }
    public void setStatus(String status)
    {
        this.status = status;
    }

    public String getStatus()
    {
        return status;
    }
    public void setPurchaseDate(Date purchaseDate)
    {
        this.purchaseDate = purchaseDate;
    }

    public Date getPurchaseDate()
    {
        return purchaseDate;
    }
    public void setPrice(BigDecimal price)
    {
        this.price = price;
    }

    public BigDecimal getPrice()
    {
        return price;
    }
    public void setLocation(String location)
    {
        this.location = location;
    }

    public String getLocation()
    {
        return location;
    }
    public void setManager(String manager)
    {
        this.manager = manager;
    }

    public String getManager()
    {
        return manager;
    }
    public void setDelFlag(String delFlag)
    {
        this.delFlag = delFlag;
    }

    public String getDelFlag()
    {
        return delFlag;
    }

    @Override
    public String toString() {
        return new ToStringBuilder(this,ToStringStyle.MULTI_LINE_STYLE)
            .append("assetId", getAssetId())
            .append("assetCode", getAssetCode())
            .append("assetName", getAssetName())
            .append("assetType", getAssetType())
            .append("status", getStatus())
            .append("purchaseDate", getPurchaseDate())
            .append("price", getPrice())
            .append("location", getLocation())
            .append("manager", getManager())
            .append("delFlag", getDelFlag())
            .append("createBy", getCreateBy())
            .append("createTime", getCreateTime())
            .append("updateBy", getUpdateBy())
            .append("updateTime", getUpdateTime())
            .append("remark", getRemark())
            .toString();
    }
}
