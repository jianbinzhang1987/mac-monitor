package com.ruoyi.monitor.domain;

import org.apache.commons.lang3.builder.ToStringBuilder;
import org.apache.commons.lang3.builder.ToStringStyle;
import com.ruoyi.common.annotation.Excel;
import com.ruoyi.common.core.domain.BaseEntity;

/**
 * 监控策略对象 monitor_policy
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
public class MonitorPolicy extends BaseEntity {
    private static final long serialVersionUID = 1L;

    /** 策略ID */
    private Long policyId;

    /** 策略名称 */
    @Excel(name = "策略名称")
    private String policyName;

    /** 流量审计规则 (JSON) */
    @Excel(name = "流量审计规则")
    private String trafficRules;

    /** 截屏策略 (JSON) */
    @Excel(name = "截屏策略")
    private String screenshotRules;

    /** 客户端系统设置 (JSON) */
    @Excel(name = "客户端系统设置")
    private String agentSettings;

    /** 是否默认策略 (0=否, 1=是) */
    @Excel(name = "是否默认策略", readConverterExp = "0=否,1=是")
    private String isDefault;

    /** 策略版本号 (通常使用时间戳或自增ID) */
    private String version;

    public void setPolicyId(Long policyId) {
        this.policyId = policyId;
    }

    public Long getPolicyId() {
        return policyId;
    }

    public void setPolicyName(String policyName) {
        this.policyName = policyName;
    }

    public String getPolicyName() {
        return policyName;
    }

    public void setTrafficRules(String trafficRules) {
        this.trafficRules = trafficRules;
    }

    public String getTrafficRules() {
        return trafficRules;
    }

    public void setScreenshotRules(String screenshotRules) {
        this.screenshotRules = screenshotRules;
    }

    public String getScreenshotRules() {
        return screenshotRules;
    }

    public void setAgentSettings(String agentSettings) {
        this.agentSettings = agentSettings;
    }

    public String getAgentSettings() {
        return agentSettings;
    }

    public void setIsDefault(String isDefault) {
        this.isDefault = isDefault;
    }

    public String getIsDefault() {
        return isDefault;
    }

    public void setVersion(String version) {
        this.version = version;
    }

    public String getVersion() {
        return version;
    }

    @Override
    public String toString() {
        return new ToStringBuilder(this, ToStringStyle.MULTI_LINE_STYLE)
                .append("policyId", getPolicyId())
                .append("policyName", getPolicyName())
                .append("trafficRules", getTrafficRules())
                .append("screenshotRules", getScreenshotRules())
                .append("agentSettings", getAgentSettings())
                .append("isDefault", getIsDefault())
                .append("version", getVersion())
                .append("createBy", getCreateBy())
                .append("createTime", getCreateTime())
                .append("updateBy", getUpdateBy())
                .append("updateTime", getUpdateTime())
                .append("remark", getRemark())
                .toString();
    }
}
