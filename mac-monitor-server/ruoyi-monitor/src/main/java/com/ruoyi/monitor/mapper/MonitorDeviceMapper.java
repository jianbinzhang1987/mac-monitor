package com.ruoyi.monitor.mapper;

import java.util.List;
import com.ruoyi.monitor.domain.MonitorDevice;

/**
 * 监控设备Mapper接口
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
public interface MonitorDeviceMapper {
    /**
     * 查询监控设备
     * 
     * @param deviceId 监控设备主键
     * @return 监控设备
     */
    public MonitorDevice selectMonitorDeviceByDeviceId(Long deviceId);

    /**
     * 根据序列号查询设备
     *
     * @param serialNumber 序列号
     * @return 监控设备
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
     * 删除监控设备
     * 
     * @param deviceId 监控设备主键
     * @return 结果
     */
    public int deleteMonitorDeviceByDeviceId(Long deviceId);

    /**
     * 批量删除监控设备
     * 
     * @param deviceIds 需要删除的数据主键集合
     * @return 结果
     */
    public int deleteMonitorDeviceByDeviceIds(Long[] deviceIds);
}
