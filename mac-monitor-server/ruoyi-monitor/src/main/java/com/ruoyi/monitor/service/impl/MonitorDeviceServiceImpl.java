package com.ruoyi.monitor.service.impl;

import java.util.List;
import java.util.UUID;

import com.ruoyi.common.utils.DateUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import com.ruoyi.monitor.mapper.MonitorDeviceMapper;
import com.ruoyi.monitor.domain.MonitorDevice;
import com.ruoyi.monitor.service.IMonitorDeviceService;

/**
 * 监控设备Service业务层处理
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
@Service
public class MonitorDeviceServiceImpl implements IMonitorDeviceService 
{
    @Autowired
    private MonitorDeviceMapper monitorDeviceMapper;

    /**
     * 查询监控设备
     * 
     * @param deviceId 监控设备主键
     * @return 监控设备
     */
    @Override
    public MonitorDevice selectMonitorDeviceByDeviceId(Long deviceId)
    {
        return monitorDeviceMapper.selectMonitorDeviceByDeviceId(deviceId);
    }

    @Override
    public MonitorDevice selectMonitorDeviceBySerialNumber(String serialNumber) {
        return monitorDeviceMapper.selectMonitorDeviceBySerialNumber(serialNumber);
    }

    /**
     * 查询监控设备列表
     * 
     * @param monitorDevice 监控设备
     * @return 监控设备
     */
    @Override
    public List<MonitorDevice> selectMonitorDeviceList(MonitorDevice monitorDevice)
    {
        return monitorDeviceMapper.selectMonitorDeviceList(monitorDevice);
    }

    /**
     * 新增监控设备
     * 
     * @param monitorDevice 监控设备
     * @return 结果
     */
    @Override
    public int insertMonitorDevice(MonitorDevice monitorDevice)
    {
        monitorDevice.setCreateTime(DateUtils.getNowDate());
        return monitorDeviceMapper.insertMonitorDevice(monitorDevice);
    }

    /**
     * 修改监控设备
     * 
     * @param monitorDevice 监控设备
     * @return 结果
     */
    @Override
    public int updateMonitorDevice(MonitorDevice monitorDevice)
    {
        monitorDevice.setUpdateTime(DateUtils.getNowDate());
        return monitorDeviceMapper.updateMonitorDevice(monitorDevice);
    }

    /**
     * 批量删除监控设备
     * 
     * @param deviceIds 需要删除的监控设备主键
     * @return 结果
     */
    @Override
    public int deleteMonitorDeviceByDeviceIds(Long[] deviceIds)
    {
        return monitorDeviceMapper.deleteMonitorDeviceByDeviceIds(deviceIds);
    }

    /**
     * 删除监控设备信息
     * 
     * @param deviceId 监控设备主键
     * @return 结果
     */
    @Override
    public int deleteMonitorDeviceByDeviceId(Long deviceId)
    {
        return monitorDeviceMapper.deleteMonitorDeviceByDeviceId(deviceId);
    }

    @Override
    public int updateMonitorDevicePolicyBatch(Long[] deviceIds, Long policyId) {
        int rows = 0;
        for (Long deviceId : deviceIds) {
            MonitorDevice device = new MonitorDevice();
            device.setDeviceId(deviceId);
            device.setPolicyId(policyId);
            device.setUpdateTime(DateUtils.getNowDate());
            rows += monitorDeviceMapper.updateMonitorDevice(device);
        }
        return rows;
    }

    /**
     * 客户端注册或更新
     */
    @Override
    public String registerOrUpdate(MonitorDevice device) {
        MonitorDevice exist = monitorDeviceMapper.selectMonitorDeviceBySerialNumber(device.getSerialNumber());
        if (exist == null) {
            device.setCreateTime(DateUtils.getNowDate());
            device.setLastHeartbeat(DateUtils.getNowDate());
            device.setStatus("1"); // 在线
            // 简单设置个默认策略版本
            if (device.getPolicyVersion() == null) {
                device.setPolicyVersion("1.0.0");
            }
            monitorDeviceMapper.insertMonitorDevice(device);
        } else {
            // Update info
            exist.setDeviceName(device.getDeviceName());
            exist.setOsVersion(device.getOsVersion());
            exist.setAppVersion(device.getAppVersion());
            exist.setRegisteredIp(device.getRegisteredIp());
            exist.setLastHeartbeat(DateUtils.getNowDate());
            exist.setStatus("1");
            monitorDeviceMapper.updateMonitorDevice(exist);
        }
        // TODO: 使用 JWT 生成真正的 Token，这里简单返回一个 UUID 模拟
        return UUID.randomUUID().toString().replace("-", "");
    }
}
