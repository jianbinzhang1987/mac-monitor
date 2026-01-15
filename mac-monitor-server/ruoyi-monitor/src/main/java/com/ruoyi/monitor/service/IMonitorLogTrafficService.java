package com.ruoyi.monitor.service;

import java.util.List;
import com.ruoyi.monitor.domain.MonitorLogTraffic;

/**
 * 流量审计日志Service接口
 *
 * @author ruoyi
 * @date 2026-01-15
 */
public interface IMonitorLogTrafficService
{
    /**
     * 查询流量审计日志
     *
     * @param logId 流量审计日志ID
     * @return 流量审计日志
     */
    public MonitorLogTraffic selectMonitorLogTrafficByLogId(Long logId);

    /**
     * 查询流量审计日志列表
     *
     * @param monitorLogTraffic 流量审计日志
     * @return 流量审计日志集合
     */
    public List<MonitorLogTraffic> selectMonitorLogTrafficList(MonitorLogTraffic monitorLogTraffic);

    /**
     * 新增流量审计日志
     *
     * @param monitorLogTraffic 流量审计日志
     * @return 结果
     */
    public int insertMonitorLogTraffic(MonitorLogTraffic monitorLogTraffic);

    /**
     * 批量删除流量审计日志
     *
     * @param logIds 需要删除的流量审计日志ID
     * @return 结果
     */
    public int deleteMonitorLogTrafficByLogIds(Long[] logIds);

    /**
     * 删除流量审计日志信息
     *
     * @param logId 流量审计日志ID
     * @return 结果
     */
    public int deleteMonitorLogTrafficByLogId(Long logId);
}
