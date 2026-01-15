package com.ruoyi.monitor.service.impl;

import java.util.List;
import com.ruoyi.common.utils.DateUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import com.ruoyi.monitor.mapper.MonitorLogBehaviorMapper;
import com.ruoyi.monitor.domain.MonitorLogBehavior;
import com.ruoyi.monitor.service.IMonitorLogBehaviorService;

/**
 * 行为审计日志Service业务层处理
 *
 * @author ruoyi
 * @date 2026-01-15
 */
@Service
public class MonitorLogBehaviorServiceImpl implements IMonitorLogBehaviorService
{
    @Autowired
    private MonitorLogBehaviorMapper monitorLogBehaviorMapper;

    /**
     * 查询行为审计日志
     *
     * @param logId 行为审计日志ID
     * @return 行为审计日志
     */
    @Override
    public MonitorLogBehavior selectMonitorLogBehaviorByLogId(Long logId)
    {
        return monitorLogBehaviorMapper.selectMonitorLogBehaviorByLogId(logId);
    }

    /**
     * 查询行为审计日志列表
     *
     * @param monitorLogBehavior 行为审计日志
     * @return 行为审计日志
     */
    @Override
    public List<MonitorLogBehavior> selectMonitorLogBehaviorList(MonitorLogBehavior monitorLogBehavior)
    {
        return monitorLogBehaviorMapper.selectMonitorLogBehaviorList(monitorLogBehavior);
    }

    /**
     * 新增行为审计日志
     *
     * @param monitorLogBehavior 行为审计日志
     * @return 结果
     */
    @Override
    public int insertMonitorLogBehavior(MonitorLogBehavior monitorLogBehavior)
    {
        monitorLogBehavior.setCreateTime(DateUtils.getNowDate());
        return monitorLogBehaviorMapper.insertMonitorLogBehavior(monitorLogBehavior);
    }

    /**
     * 批量删除行为审计日志
     *
     * @param logIds 需要删除的行为审计日志ID
     * @return 结果
     */
    @Override
    public int deleteMonitorLogBehaviorByLogIds(Long[] logIds)
    {
        return monitorLogBehaviorMapper.deleteMonitorLogBehaviorByLogIds(logIds);
    }

    /**
     * 删除行为审计日志信息
     *
     * @param logId 行为审计日志ID
     * @return 结果
     */
    @Override
    public int deleteMonitorLogBehaviorByLogId(Long logId)
    {
        return monitorLogBehaviorMapper.deleteMonitorLogBehaviorByLogId(logId);
    }
}
