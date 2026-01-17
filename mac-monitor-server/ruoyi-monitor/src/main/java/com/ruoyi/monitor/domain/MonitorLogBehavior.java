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
public class MonitorLogBehavior extends BaseEntity {
    private static final long serialVersionUID = 1L;

    /** 日志ID */
    private Long logId;

    /** 设备序列号 */
    @Excel(name = "设备序列号")
    @com.fasterxml.jackson.annotation.JsonAlias("cpe_id")
    private String serialNumber;

    /** 事件时间 */
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    @Excel(name = "事件时间", width = 30, dateFormat = "yyyy-MM-dd HH:mm:ss")
    @com.fasterxml.jackson.annotation.JsonAlias("op_time")
    private Date eventTime;

    /** 事件类型 (PROCESS_LAUNCH, USB_INSERT, FILE_CHANGE) */
    @Excel(name = "事件类型")
    @com.fasterxml.jackson.annotation.JsonAlias("op_type")
    private String eventType;

    /** 相关进程 */
    @Excel(name = "相关进程")
    @com.fasterxml.jackson.annotation.JsonAlias("proc")
    private String processName;

    /** 详细描述 (JSON) */
    @Excel(name = "详细描述")
    private String detail;

    /** 风险等级 (0-2) */
    @Excel(name = "风险等级")
    @com.fasterxml.jackson.annotation.JsonAlias("risk_level")
    private Integer riskLevel;

    /** 主机ID */
    @Excel(name = "主机ID")
    @com.fasterxml.jackson.annotation.JsonAlias("host_id")
    private String hostId;

    /** MAC地址 */
    @Excel(name = "MAC地址")
    private String mac;

    /** IP地址 */
    @Excel(name = "IP地址")
    private String ip;

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

    public void setEventTime(Date eventTime) {
        this.eventTime = eventTime;
    }

    public Date getEventTime() {
        return eventTime;
    }

    public void setEventType(String eventType) {
        this.eventType = eventType;
    }

    public String getEventType() {
        return eventType;
    }

    public void setProcessName(String processName) {
        this.processName = processName;
    }

    public String getProcessName() {
        return processName;
    }

    public void setDetail(String detail) {
        this.detail = detail;
    }

    public String getDetail() {
        return detail;
    }

    public void setRiskLevel(Integer riskLevel) {
        this.riskLevel = riskLevel;
    }

    public Integer getRiskLevel() {
        return riskLevel;
    }

    public void setHostId(String hostId) {
        this.hostId = hostId;
    }

    public String getHostId() {
        return hostId;
    }

    public void setMac(String mac) {
        this.mac = mac;
    }

    public String getMac() {
        return mac;
    }

    public void setIp(String ip) {
        this.ip = ip;
    }

    public String getIp() {
        return ip;
    }

    @Override
    public String toString() {
        return new ToStringBuilder(this, ToStringStyle.MULTI_LINE_STYLE)
                .append("logId", getLogId())
                .append("serialNumber", getSerialNumber())
                .append("eventTime", getEventTime())
                .append("eventType", getEventType())
                .append("processName", getProcessName())
                .append("detail", getDetail())
                .append("riskLevel", getRiskLevel())
                .append("hostId", getHostId())
                .append("mac", getMac())
                .append("ip", getIp())
                .append("createTime", getCreateTime())
                .toString();
    }
}
