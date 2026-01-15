package com.ruoyi.monitor.service.impl;

import java.util.List;
import com.ruoyi.common.utils.DateUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import com.ruoyi.monitor.mapper.MonitorLogTrafficMapper;
import com.ruoyi.monitor.domain.MonitorLogTraffic;
import com.ruoyi.monitor.service.IMonitorLogTrafficService;

/**
 * 流量审计日志Service业务层处理
 *
 * @author ruoyi
 * @date 2026-01-15
 */
@Service
public class MonitorLogTrafficServiceImpl implements IMonitorLogTrafficService
{
    @Autowired
    private MonitorLogTrafficMapper monitorLogTrafficMapper;

    /**
     * 查询流量审计日志
     *
     * @param logId 流量审计日志ID
     * @return 流量审计日志
     */
    @Override
    public MonitorLogTraffic selectMonitorLogTrafficByLogId(Long logId)
    {
        return monitorLogTrafficMapper.selectMonitorLogTrafficByLogId(logId);
    }

    /**
     * 查询流量审计日志列表
     *
     * @param monitorLogTraffic 流量审计日志
     * @return 流量审计日志
     */
    @Override
    public List<MonitorLogTraffic> selectMonitorLogTrafficList(MonitorLogTraffic monitorLogTraffic)
    {
        return monitorLogTrafficMapper.selectMonitorLogTrafficList(monitorLogTraffic);
    }

    /**
     * 新增流量审计日志
     *
     * @param monitorLogTraffic 流量审计日志
     * @return 结果
     */
    @Override
    public int insertMonitorLogTraffic(MonitorLogTraffic monitorLogTraffic)
    {
        monitorLogTraffic.setCreateTime(DateUtils.getNowDate());
        return monitorLogTrafficMapper.insertMonitorLogTraffic(monitorLogTraffic);
    }

    /**
     * 批量删除流量审计日志
     *
     * @param logIds 需要删除的流量审计日志ID
     * @return 结果
     */
    @Override
    public int deleteMonitorLogTrafficByLogIds(Long[] logIds)
    {
        return monitorLogTrafficMapper.deleteMonitorLogTrafficByLogIds(logIds);
    }

    /**
     * 删除流量审计日志信息
     *
     * @param logId 流量审计日志ID
     * @return 结果
     */
    @Override
    public int deleteMonitorLogTrafficByLogId(Long logId)
    {
        return monitorLogTrafficMapper.deleteMonitorLogTrafficByLogId(logId);
    }
}
