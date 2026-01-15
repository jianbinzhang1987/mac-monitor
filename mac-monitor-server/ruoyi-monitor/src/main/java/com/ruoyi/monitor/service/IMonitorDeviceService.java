package com.ruoyi.monitor.service;

import java.util.List;
import com.ruoyi.monitor.domain.MonitorDevice;

/**
 * 监控设备Service接口
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
public interface IMonitorDeviceService 
{
    /**
     * 查询监控设备
     * 
     * @param deviceId 监控设备主键
     * @return 监控设备
     */
    public MonitorDevice selectMonitorDeviceByDeviceId(Long deviceId);

    /**
     * 根据序列号查询设备
     * @param serialNumber
     * @return
     */
    public MonitorDevice selectMonitorDeviceBySerialNumber(String serialNumber);

    /**
     * 查询监控设备列表
     * 
     * @param monitorDevice 监控设备
     * @return 监控设备集合
     */
    public List<MonitorDevice> selectMonitorDeviceList(MonitorDevice monitorDevice);

    /**
     * 新增监控设备
     * 
     * @param monitorDevice 监控设备
     * @return 结果
     */
    public int insertMonitorDevice(MonitorDevice monitorDevice);

    /**
     * 修改监控设备
     * 
     * @param monitorDevice 监控设备
     * @return 结果
     */
    public int updateMonitorDevice(MonitorDevice monitorDevice);

    /**
     * 批量删除监控设备
     * 
     * @param deviceIds 需要删除的监控设备主键集合
     * @return 结果
     */
    public int deleteMonitorDeviceByDeviceIds(Long[] deviceIds);

    /**
     * 删除监控设备信息
     * 
     * @param deviceId 监控设备主键
     * @return 结果
     */
    public int deleteMonitorDeviceByDeviceId(Long deviceId);

    /**
     * 批量修改设备关联策略
     *
     * @param deviceIds 需要修改的设备主键集合
     * @param policyId 策略ID
     * @return 结果
     */
    public int updateMonitorDevicePolicyBatch(Long[] deviceIds, Long policyId);

    /**
     * 客户端注册或心跳更新
     * @param device 设备信息
     * @return Token
     */
    public String registerOrUpdate(MonitorDevice device);
}
