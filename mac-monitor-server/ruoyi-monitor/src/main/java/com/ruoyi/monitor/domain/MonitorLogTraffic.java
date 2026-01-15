package com.ruoyi.monitor.domain;

import com.fasterxml.jackson.annotation.JsonFormat;
import com.ruoyi.common.annotation.Excel;
import com.ruoyi.common.core.domain.BaseEntity;
import org.apache.commons.lang3.builder.ToStringBuilder;
import org.apache.commons.lang3.builder.ToStringStyle;
import java.util.Date;

/**
 * 流量审计日志对象 monitor_log_traffic
 *
 * @author ruoyi
 * @date 2026-01-15
 */
public class MonitorLogTraffic extends BaseEntity {
    private static final long serialVersionUID = 1L;

    /** 日志ID */
    private Long logId;

    /** 设备序列号 */
    @Excel(name = "设备序列号")
    @com.fasterxml.jackson.annotation.JsonProperty("cpe_id")
    private String serialNumber;

    /** 审计时间 */
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    @Excel(name = "审计时间", width = 30, dateFormat = "yyyy-MM-dd HH:mm:ss")
    @com.fasterxml.jackson.annotation.JsonProperty("req_time")
    private Date auditTime;

    /** 请求URL */
    @Excel(name = "请求URL")
    @com.fasterxml.jackson.annotation.JsonProperty("url")
    private String url;

    /** HTTP方法 */
    @Excel(name = "HTTP方法")
    @com.fasterxml.jackson.annotation.JsonProperty("method_type")
    private String method;

    /** 域名 */
    @Excel(name = "域名")
    @com.fasterxml.jackson.annotation.JsonProperty("domain")
    private String domain;

    /** 进程名 */
    @Excel(name = "进程名")
    @com.fasterxml.jackson.annotation.JsonProperty("process_name")
    private String processName;

    /** 风险等级 (0-2) */
    @Excel(name = "风险等级")
    @com.fasterxml.jackson.annotation.JsonProperty("risk_level")
    private Integer riskLevel;

    public void setLogId(Long logId) {
        this.logId = logId;
    }

    public Long getLogId() {
        return logId;
    }

    public void setSerialNumber(String serialNumber) {
        this.serialNumber = serialNumber;
    }

    public String getSerialNumber() {
        return serialNumber;
    }

    public void setAuditTime(Date auditTime) {
        this.auditTime = auditTime;
    }

    public Date getAuditTime() {
        return auditTime;
    }

    public void setUrl(String url) {
        this.url = url;
    }

    public String getUrl() {
        return url;
    }

    public void setMethod(String method) {
        this.method = method;
    }

    public String getMethod() {
        return method;
    }

    public void setDomain(String domain) {
        this.domain = domain;
    }

    public String getDomain() {
        return domain;
    }

    public void setProcessName(String processName) {
        this.processName = processName;
    }

    public String getProcessName() {
        return processName;
    }

    public void setRiskLevel(Integer riskLevel) {
        this.riskLevel = riskLevel;
    }

    public Integer getRiskLevel() {
        return riskLevel;
    }

    @Override
    public String toString() {
        return new ToStringBuilder(this, ToStringStyle.MULTI_LINE_STYLE)
                .append("logId", getLogId())
                .append("serialNumber", getSerialNumber())
                .append("auditTime", getAuditTime())
                .append("url", getUrl())
                .append("method", getMethod())
                .append("domain", getDomain())
                .append("processName", getProcessName())
                .append("riskLevel", getRiskLevel())
                .append("createTime", getCreateTime())
                .toString();
    }
}
