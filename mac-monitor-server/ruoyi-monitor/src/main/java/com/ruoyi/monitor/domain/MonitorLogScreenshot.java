package com.ruoyi.monitor.domain;

import java.util.Date;
import com.fasterxml.jackson.annotation.JsonFormat;
import org.apache.commons.lang3.builder.ToStringBuilder;
import org.apache.commons.lang3.builder.ToStringStyle;
import com.ruoyi.common.annotation.Excel;
import com.ruoyi.common.core.domain.BaseEntity;

/**
 * 截图日志对象 monitor_log_screenshot
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
public class MonitorLogScreenshot extends BaseEntity {
    private static final long serialVersionUID = 1L;

    /** 日志ID */
    private Long logId;

    /** 设备ID */
    @Excel(name = "设备ID")
    private Long deviceId;

    /** 设备序列号 */
    @Excel(name = "设备序列号")
    @com.fasterxml.jackson.annotation.JsonAlias("cpe_id")
    private String serialNumber;

    /** 截图时间 */
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    @Excel(name = "截图时间", width = 30, dateFormat = "yyyy-MM-dd HH:mm:ss")
    @com.fasterxml.jackson.annotation.JsonAlias("capture_time")
    private Date captureTime;

    /** 文件路径 */
    @Excel(name = "文件路径")
    @com.fasterxml.jackson.annotation.JsonAlias("image_path")
    private String filePath;

    /** OCR识别文本 */
    @Excel(name = "OCR识别文本")
    @com.fasterxml.jackson.annotation.JsonAlias("ocr_text")
    private String ocrText;

    /** 风险等级(0-2) */
    @Excel(name = "风险等级(0-2)")
    @com.fasterxml.jackson.annotation.JsonAlias("is_sensitive")
    private Integer riskLevel;

    /** 主机ID */
    @Excel(name = "主机ID")
    @com.fasterxml.jackson.annotation.JsonAlias("host_id")
    private String hostId;

    /** MAC地址 */
    @Excel(name = "MAC地址")
    @com.fasterxml.jackson.annotation.JsonAlias("mac")
    private String mac;

    /** IP地址 */
    @Excel(name = "IP地址")
    @com.fasterxml.jackson.annotation.JsonAlias("ip")
    private String ip;

    /** 应用名称 */
    @Excel(name = "应用名称")
    @com.fasterxml.jackson.annotation.JsonAlias("app_name")
    private String appName;

    /** 图片哈希 */
    @Excel(name = "图片哈希")
    @com.fasterxml.jackson.annotation.JsonAlias("image_hash")
    private String imageHash;

    public void setLogId(Long logId) {
        this.logId = logId;
    }

    public Long getLogId() {
        return logId;
    }

    public void setDeviceId(Long deviceId) {
        this.deviceId = deviceId;
    }

    public Long getDeviceId() {
        return deviceId;
    }

    public void setSerialNumber(String serialNumber) {
        this.serialNumber = serialNumber;
    }

    public String getSerialNumber() {
        return serialNumber;
    }

    public void setCaptureTime(Date captureTime) {
        this.captureTime = captureTime;
    }

    public Date getCaptureTime() {
        return captureTime;
    }

    public void setFilePath(String filePath) {
        this.filePath = filePath;
    }

    public String getFilePath() {
        return filePath;
    }

    public void setOcrText(String ocrText) {
        this.ocrText = ocrText;
    }

    public String getOcrText() {
        return ocrText;
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

    public void setAppName(String appName) {
        this.appName = appName;
    }

    public String getAppName() {
        return appName;
    }

    public void setImageHash(String imageHash) {
        this.imageHash = imageHash;
    }

    public String getImageHash() {
        return imageHash;
    }

    @Override
    public String toString() {
        return new ToStringBuilder(this, ToStringStyle.MULTI_LINE_STYLE)
                .append("logId", getLogId())
                .append("deviceId", getDeviceId())
                .append("serialNumber", getSerialNumber())
                .append("captureTime", getCaptureTime())
                .append("filePath", getFilePath())
                .append("ocrText", getOcrText())
                .append("riskLevel", getRiskLevel())
                .append("hostId", getHostId())
                .append("mac", getMac())
                .append("ip", getIp())
                .append("appName", getAppName())
                .append("imageHash", getImageHash())
                .append("createTime", getCreateTime())
                .toString();
    }
}
