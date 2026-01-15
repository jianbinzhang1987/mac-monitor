package com.ruoyi.monitor.domain;

import com.fasterxml.jackson.annotation.JsonFormat;
import com.ruoyi.common.annotation.Excel;
import com.ruoyi.common.core.domain.BaseEntity;
import org.apache.commons.lang3.builder.ToStringBuilder;
import org.apache.commons.lang3.builder.ToStringStyle;
import java.util.Date;

/**
 * 监控设备对象 monitor_device
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
public class MonitorDevice extends BaseEntity
{
    private static final long serialVersionUID = 1L;

    /** 设备ID */
    private Long deviceId;

    /** 序列号 */
    @Excel(name = "序列号")
    private String serialNumber;

    /** 设备名称 */
    @Excel(name = "设备名称")
    private String deviceName;

    /** 系统版本 */
    @Excel(name = "系统版本")
    private String osVersion;

    /** 客户端版本 */
    @Excel(name = "客户端版本")
    private String appVersion;

    /** 在线状态 (0离线 1在线) */
    @Excel(name = "在线状态", readConverterExp = "0=离线,1=在线")
    private String status;

    /** 最后心跳时间 */
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    @Excel(name = "最后心跳时间", width = 30, dateFormat = "yyyy-MM-dd HH:mm:ss")
    private Date lastHeartbeat;

    /** 策略版本 */
    @Excel(name = "策略版本")
    private String policyVersion;

    /** 关联策略ID */
    private Long policyId;

    /** 目标策略版本 (用于界面对比) */
    private String targetVersion;

    /** 注册IP */
    @Excel(name = "注册IP")
    private String registeredIp;

    /** 设备ID组 */
    private Long[] deviceIds;

    public void setDeviceId(Long deviceId)
    {
        this.deviceId = deviceId;
    }

    public Long getDeviceId()
    {
        return deviceId;
    }

    public void setPolicyId(Long policyId)
    {
        this.policyId = policyId;
    }

    public Long getPolicyId()
    {
        return policyId;
    }

    public void setTargetVersion(String targetVersion)
    {
        this.targetVersion = targetVersion;
    }

    public String getTargetVersion()
    {
        return targetVersion;
    }

    public void setSerialNumber(String serialNumber)
    {
        this.serialNumber = serialNumber;
    }

    public String getSerialNumber() 
    {
        return serialNumber;
    }
    public void setDeviceName(String deviceName) 
    {
        this.deviceName = deviceName;
    }

    public String getDeviceName() 
    {
        return deviceName;
    }
    public void setOsVersion(String osVersion) 
    {
        this.osVersion = osVersion;
    }

    public String getOsVersion() 
    {
        return osVersion;
    }
    public void setAppVersion(String appVersion) 
    {
        this.appVersion = appVersion;
    }

    public String getAppVersion() 
    {
        return appVersion;
    }
    public void setStatus(String status) 
    {
        this.status = status;
    }

    public String getStatus() 
    {
        return status;
    }
    public void setLastHeartbeat(Date lastHeartbeat) 
    {
        this.lastHeartbeat = lastHeartbeat;
    }

    public Date getLastHeartbeat() 
    {
        return lastHeartbeat;
    }
    public void setPolicyVersion(String policyVersion) 
    {
        this.policyVersion = policyVersion;
    }

    public String getPolicyVersion() 
    {
        return policyVersion;
    }
    public void setRegisteredIp(String registeredIp) 
    {
        this.registeredIp = registeredIp;
    }

    public String getRegisteredIp()
    {
        return registeredIp;
    }

    public Long[] getDeviceIds()
    {
        return deviceIds;
    }

    public void setDeviceIds(Long[] deviceIds)
    {
        this.deviceIds = deviceIds;
    }

    @Override
    public String toString() {
        return new ToStringBuilder(this,ToStringStyle.MULTI_LINE_STYLE)
            .append("deviceId", getDeviceId())
            .append("serialNumber", getSerialNumber())
            .append("deviceName", getDeviceName())
            .append("osVersion", getOsVersion())
            .append("appVersion", getAppVersion())
            .append("status", getStatus())
            .append("lastHeartbeat", getLastHeartbeat())
            .append("policyVersion", getPolicyVersion())
            .append("registeredIp", getRegisteredIp())
            .append("createBy", getCreateBy())
            .append("createTime", getCreateTime())
            .append("updateBy", getUpdateBy())
            .append("updateTime", getUpdateTime())
            .append("remark", getRemark())
            .toString();
    }
}
