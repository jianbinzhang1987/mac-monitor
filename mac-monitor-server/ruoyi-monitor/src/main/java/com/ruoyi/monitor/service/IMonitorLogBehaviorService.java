package com.ruoyi.monitor.service;

import java.util.List;
import com.ruoyi.monitor.domain.MonitorLogBehavior;

/**
 * 行为审计日志Service接口
 *
 * @author ruoyi
 * @date 2026-01-15
 */
public interface IMonitorLogBehaviorService
{
    /**
     * 查询行为审计日志
     *
     * @param logId 行为审计日志ID
     * @return 行为审计日志
     */
    public MonitorLogBehavior selectMonitorLogBehaviorByLogId(Long logId);

    /**
     * 查询行为审计日志列表
     *
     * @param monitorLogBehavior 行为审计日志
     * @return 行为审计日志集合
     */
    public List<MonitorLogBehavior> selectMonitorLogBehaviorList(MonitorLogBehavior monitorLogBehavior);

    /**
     * 新增行为审计日志
     *
     * @param monitorLogBehavior 行为审计日志
     * @return 结果
     */
    public int insertMonitorLogBehavior(MonitorLogBehavior monitorLogBehavior);

    /**
     * 批量删除行为审计日志
     *
     * @param logIds 需要删除的行为审计日志ID
     * @return 结果
     */
    public int deleteMonitorLogBehaviorByLogIds(Long[] logIds);

    /**
     * 删除行为审计日志信息
     *
     * @param logId 行为审计日志ID
     * @return 结果
     */
    public int deleteMonitorLogBehaviorByLogId(Long logId);
}
