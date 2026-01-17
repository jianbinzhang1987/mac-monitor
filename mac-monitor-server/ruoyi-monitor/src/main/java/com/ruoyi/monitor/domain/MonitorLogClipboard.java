package com.ruoyi.monitor.domain;

import java.util.Date;
import com.fasterxml.jackson.annotation.JsonFormat;
import org.apache.commons.lang3.builder.ToStringBuilder;
import org.apache.commons.lang3.builder.ToStringStyle;
import com.ruoyi.common.annotation.Excel;
import com.ruoyi.common.core.domain.BaseEntity;

/**
 * 剪贴板审计日志对象 monitor_log_clipboard
 *
 * @author ruoyi
 * @date 2026-01-17
 */
public class MonitorLogClipboard extends BaseEntity {
    private static final long serialVersionUID = 1L;

    /** 日志ID */
    private Long logId;

    /** 设备序列号 */
    @Excel(name = "设备序列号")
    @com.fasterxml.jackson.annotation.JsonAlias("cpe_id")
    private String serialNumber;

    /** 设备ID */
    private Long deviceId;

    /** 操作时间 */
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    @Excel(name = "操作时间", width = 30, dateFormat = "yyyy-MM-dd HH:mm:ss")
    @com.fasterxml.jackson.annotation.JsonAlias("op_time")
    private Date opTime;

    /** 应用名称 */
    @Excel(name = "应用名称")
    @com.fasterxml.jackson.annotation.JsonAlias("app_name")
    private String appName;

    /** 应用包名 */
    @Excel(name = "应用包名")
    @com.fasterxml.jackson.annotation.JsonAlias("bundle_id")
    private String bundleId;

    /** 剪贴板内容 */
    @Excel(name = "剪贴板内容")
    private String content;

    /** 内容类型 */
    @Excel(name = "内容类型")
    @com.fasterxml.jackson.annotation.JsonAlias("content_type")
    private String contentType;

    /** 风险等级(0-2) */
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

    public void setDeviceId(Long deviceId) {
        this.deviceId = deviceId;
    }

    public Long getDeviceId() {
        return deviceId;
    }

    public void setOpTime(Date opTime) {
        this.opTime = opTime;
    }

    public Date getOpTime() {
        return opTime;
    }

    public void setAppName(String appName) {
        this.appName = appName;
    }

    public String getAppName() {
        return appName;
    }

    public void setBundleId(String bundleId) {
        this.bundleId = bundleId;
    }

    public String getBundleId() {
        return bundleId;
    }

    public void setContent(String content) {
        this.content = content;
    }

    public String getContent() {
        return content;
    }

    public void setContentType(String contentType) {
        this.contentType = contentType;
    }

    public String getContentType() {
        return contentType;
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
                .append("deviceId", getDeviceId())
                .append("opTime", getOpTime())
                .append("appName", getAppName())
                .append("bundleId", getBundleId())
                .append("content", getContent())
                .append("contentType", getContentType())
                .append("riskLevel", getRiskLevel())
                .append("hostId", getHostId())
                .append("mac", getMac())
                .append("ip", getIp())
                .append("createTime", getCreateTime())
                .toString();
    }
}
