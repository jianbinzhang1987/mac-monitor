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
public class MonitorLogScreenshot extends BaseEntity
{
    private static final long serialVersionUID = 1L;

    /** 日志ID */
    private Long logId;

    /** 设备ID */
    @Excel(name = "设备ID")
    private Long deviceId;

    /** 设备序列号 */
    @Excel(name = "设备序列号")
    private String serialNumber;

    /** 截图时间 */
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    @Excel(name = "截图时间", width = 30, dateFormat = "yyyy-MM-dd HH:mm:ss")
    private Date captureTime;

    /** 文件路径 */
    @Excel(name = "文件路径")
    private String filePath;

    /** OCR识别文本 */
    @Excel(name = "OCR识别文本")
    private String ocrText;

    /** 风险等级(0-2) */
    @Excel(name = "风险等级(0-2)")
    private Integer riskLevel;

    public void setLogId(Long logId) 
    {
        this.logId = logId;
    }

    public Long getLogId() 
    {
        return logId;
    }
    public void setDeviceId(Long deviceId) 
    {
        this.deviceId = deviceId;
    }

    public Long getDeviceId() 
    {
        return deviceId;
    }
    public void setSerialNumber(String serialNumber) 
    {
        this.serialNumber = serialNumber;
    }

    public String getSerialNumber() 
    {
        return serialNumber;
    }
    public void setCaptureTime(Date captureTime) 
    {
        this.captureTime = captureTime;
    }

    public Date getCaptureTime() 
    {
        return captureTime;
    }
    public void setFilePath(String filePath) 
    {
        this.filePath = filePath;
    }

    public String getFilePath() 
    {
        return filePath;
    }
    public void setOcrText(String ocrText) 
    {
        this.ocrText = ocrText;
    }

    public String getOcrText() 
    {
        return ocrText;
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
            .append("deviceId", getDeviceId())
            .append("serialNumber", getSerialNumber())
            .append("captureTime", getCaptureTime())
            .append("filePath", getFilePath())
            .append("ocrText", getOcrText())
            .append("riskLevel", getRiskLevel())
            .append("createTime", getCreateTime())
            .toString();
    }
}
