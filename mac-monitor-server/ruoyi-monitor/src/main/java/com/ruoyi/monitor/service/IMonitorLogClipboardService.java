package com.ruoyi.monitor.service;

import java.util.List;
import com.ruoyi.monitor.domain.MonitorLogClipboard;

/**
 * 剪贴板审计日志Service接口
 *
 * @author ruoyi
 * @date 2026-01-17
 */
public interface IMonitorLogClipboardService
{
    /**
     * 查询剪贴板审计日志
     *
     * @param logId 剪贴板审计日志主键
     * @return 剪贴板审计日志
     */
    public MonitorLogClipboard selectMonitorLogClipboardByLogId(Long logId);

    /**
     * 查询剪贴板审计日志列表
     *
     * @param monitorLogClipboard 剪贴板审计日志
     * @return 剪贴板审计日志集合
     */
    public List<MonitorLogClipboard> selectMonitorLogClipboardList(MonitorLogClipboard monitorLogClipboard);

    /**
     * 新增剪贴板审计日志
     *
     * @param monitorLogClipboard 剪贴板审计日志
     * @return 结果
     */
    public int insertMonitorLogClipboard(MonitorLogClipboard monitorLogClipboard);

    /**
     * 修改剪贴板审计日志
     *
     * @param monitorLogClipboard 剪贴板审计日志
     * @return 结果
     */
    public int updateMonitorLogClipboard(MonitorLogClipboard monitorLogClipboard);

    /**
     * 批量删除剪贴板审计日志
     *
     * @param logIds 需要删除的剪贴板审计日志主键集合
     * @return 结果
     */
    public int deleteMonitorLogClipboardByLogIds(Long[] logIds);

    /**
     * 删除剪贴板审计日志信息
     *
     * @param logId 剪贴板审计日志主键
     * @return 结果
     */
    public int deleteMonitorLogClipboardByLogId(Long logId);
}
