package com.ruoyi.monitor.mapper;

import java.util.List;
import com.ruoyi.monitor.domain.MonitorLogScreenshot;

/**
 * 截图日志Mapper接口
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
public interface MonitorLogScreenshotMapper 
{
    /**
     * 查询截图日志
     * 
     * @param logId 截图日志ID
     * @return 截图日志
     */
    public MonitorLogScreenshot selectMonitorLogScreenshotByLogId(Long logId);

    /**
     * 查询截图日志列表
     * 
     * @param monitorLogScreenshot 截图日志
     * @return 截图日志集合
     */
    public List<MonitorLogScreenshot> selectMonitorLogScreenshotList(MonitorLogScreenshot monitorLogScreenshot);

    /**
     * 新增截图日志
     * 
     * @param monitorLogScreenshot 截图日志
     * @return 结果
     */
    public int insertMonitorLogScreenshot(MonitorLogScreenshot monitorLogScreenshot);

    /**
     * 修改截图日志
     * 
     * @param monitorLogScreenshot 截图日志
     * @return 结果
     */
    public int updateMonitorLogScreenshot(MonitorLogScreenshot monitorLogScreenshot);

    /**
     * 删除截图日志
     * 
     * @param logId 截图日志ID
     * @return 结果
     */
    public int deleteMonitorLogScreenshotByLogId(Long logId);

    /**
     * 批量删除截图日志
     * 
     * @param logIds 需要删除的数据ID
     * @return 结果
     */
    public int deleteMonitorLogScreenshotByLogIds(Long[] logIds);
}
