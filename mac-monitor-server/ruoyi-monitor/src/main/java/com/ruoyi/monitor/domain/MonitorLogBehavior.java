package com.ruoyi.monitor.domain;

import com.fasterxml.jackson.annotation.JsonFormat;
import com.ruoyi.common.annotation.Excel;
import com.ruoyi.common.core.domain.BaseEntity;
import org.apache.commons.lang3.builder.ToStringBuilder;
import org.apache.commons.lang3.builder.ToStringStyle;
import java.util.Date;

/**
 * 行为审计日志对象 monitor_log_behavior
 *
 * @author ruoyi
 * @date 2026-01-15
 */
public class MonitorLogBehavior extends BaseEntity
{
    private static final long serialVersionUID = 1L;

    /** 日志ID */
    private Long logId;

    /** 设备序列号 */
    @Excel(name = "设备序列号")
    private String serialNumber;

    /** 事件时间 */
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    @Excel(name = "事件时间", width = 30, dateFormat = "yyyy-MM-dd HH:mm:ss")
    private Date eventTime;

    /** 事件类型 (PROCESS_LAUNCH, USB_INSERT, FILE_CHANGE) */
    @Excel(name = "事件类型")
    private String eventType;

    /** 相关进程 */
    @Excel(name = "相关进程")
    private String processName;

    /** 详细描述 (JSON) */
    @Excel(name = "详细描述")
    private String detail;

    /** 风险等级 (0-2) */
    @Excel(name = "风险等级")
    private Integer riskLevel;

    public void setLogId(Long logId)
    {
        this.logId = logId;
    }

    public Long getLogId()
    {
        return logId;
    }
    public void setSerialNumber(String serialNumber)
    {
        this.serialNumber = serialNumber;
    }

    public String getSerialNumber()
    {
        return serialNumber;
    }
    public void setEventTime(Date eventTime)
    {
        this.eventTime = eventTime;
    }

    public Date getEventTime()
    {
        return eventTime;
    }
    public void setEventType(String eventType)
    {
        this.eventType = eventType;
    }

    public String getEventType()
    {
        return eventType;
    }
    public void setProcessName(String processName)
    {
        this.processName = processName;
    }

    public String getProcessName()
    {
        return processName;
    }
    public void setDetail(String detail)
    {
        this.detail = detail;
    }

    public String getDetail()
    {
        return detail;
    }
    public void setRiskLevel(Integer riskLevel)
    {
        this.riskLevel = riskLevel;
    }

    public Integer getRiskLevel()
    {
        return riskLevel;
    }

    @Override
    public String toString() {
        return new ToStringBuilder(this,ToStringStyle.MULTI_LINE_STYLE)
            .append("logId", getLogId())
            .append("serialNumber", getSerialNumber())
            .append("eventTime", getEventTime())
            .append("eventType", getEventType())
            .append("processName", getProcessName())
            .append("detail", getDetail())
            .append("riskLevel", getRiskLevel())
            .append("createTime", getCreateTime())
            .toString();
    }
}
