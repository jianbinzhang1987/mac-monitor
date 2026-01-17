package com.ruoyi.monitor.service.impl;

import java.util.List;
import com.ruoyi.common.utils.DateUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import com.ruoyi.monitor.mapper.MonitorLogClipboardMapper;
import com.ruoyi.monitor.domain.MonitorLogClipboard;
import com.ruoyi.monitor.service.IMonitorLogClipboardService;

/**
 * 剪贴板审计日志Service业务层处理
 *
 * @author ruoyi
 * @date 2026-01-17
 */
@Service
public class MonitorLogClipboardServiceImpl implements IMonitorLogClipboardService
{
    @Autowired
    private MonitorLogClipboardMapper monitorLogClipboardMapper;

    /**
     * 查询剪贴板审计日志
     *
     * @param logId 剪贴板审计日志主键
     * @return 剪贴板审计日志
     */
    @Override
    public MonitorLogClipboard selectMonitorLogClipboardByLogId(Long logId)
    {
        return monitorLogClipboardMapper.selectMonitorLogClipboardByLogId(logId);
    }

    /**
     * 查询剪贴板审计日志列表
     *
     * @param monitorLogClipboard 剪贴板审计日志
     * @return 剪贴板审计日志
     */
    @Override
    public List<MonitorLogClipboard> selectMonitorLogClipboardList(MonitorLogClipboard monitorLogClipboard)
    {
        return monitorLogClipboardMapper.selectMonitorLogClipboardList(monitorLogClipboard);
    }

    /**
     * 新增剪贴板审计日志
     *
     * @param monitorLogClipboard 剪贴板审计日志
     * @return 结果
     */
    @Override
    public int insertMonitorLogClipboard(MonitorLogClipboard monitorLogClipboard)
    {
        monitorLogClipboard.setCreateTime(DateUtils.getNowDate());
        return monitorLogClipboardMapper.insertMonitorLogClipboard(monitorLogClipboard);
    }

    /**
     * 修改剪贴板审计日志
     *
     * @param monitorLogClipboard 剪贴板审计日志
     * @return 结果
     */
    @Override
    public int updateMonitorLogClipboard(MonitorLogClipboard monitorLogClipboard)
    {
        return monitorLogClipboardMapper.updateMonitorLogClipboard(monitorLogClipboard);
    }

    /**
     * 批量删除剪贴板审计日志
     *
     * @param logIds 需要删除的剪贴板审计日志主键
     * @return 结果
     */
    @Override
    public int deleteMonitorLogClipboardByLogIds(Long[] logIds)
    {
        return monitorLogClipboardMapper.deleteMonitorLogClipboardByLogIds(logIds);
    }

    /**
     * 删除剪贴板审计日志信息
     *
     * @param logId 剪贴板审计日志主键
     * @return 结果
     */
    @Override
    public int deleteMonitorLogClipboardByLogId(Long logId)
    {
        return monitorLogClipboardMapper.deleteMonitorLogClipboardByLogId(logId);
    }
}
