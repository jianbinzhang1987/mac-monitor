package com.ruoyi.monitor.service.impl;

import java.util.List;
import com.ruoyi.common.utils.DateUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import com.ruoyi.monitor.mapper.MonitorLogScreenshotMapper;
import com.ruoyi.monitor.domain.MonitorLogScreenshot;
import com.ruoyi.monitor.service.IMonitorLogScreenshotService;

/**
 * 截图日志Service业务层处理
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
@Service
public class MonitorLogScreenshotServiceImpl implements IMonitorLogScreenshotService 
{
    @Autowired
    private MonitorLogScreenshotMapper monitorLogScreenshotMapper;

    /**
     * 查询截图日志
     * 
     * @param logId 截图日志ID
     * @return 截图日志
     */
    @Override
    public MonitorLogScreenshot selectMonitorLogScreenshotByLogId(Long logId)
    {
        return monitorLogScreenshotMapper.selectMonitorLogScreenshotByLogId(logId);
    }

    /**
     * 查询截图日志列表
     * 
     * @param monitorLogScreenshot 截图日志
     * @return 截图日志
     */
    @Override
    public List<MonitorLogScreenshot> selectMonitorLogScreenshotList(MonitorLogScreenshot monitorLogScreenshot)
    {
        return monitorLogScreenshotMapper.selectMonitorLogScreenshotList(monitorLogScreenshot);
    }

    /**
     * 新增截图日志
     * 
     * @param monitorLogScreenshot 截图日志
     * @return 结果
     */
    @Override
    public int insertMonitorLogScreenshot(MonitorLogScreenshot monitorLogScreenshot)
    {
        monitorLogScreenshot.setCreateTime(DateUtils.getNowDate());
        return monitorLogScreenshotMapper.insertMonitorLogScreenshot(monitorLogScreenshot);
    }

    /**
     * 修改截图日志
     * 
     * @param monitorLogScreenshot 截图日志
     * @return 结果
     */
    @Override
    public int updateMonitorLogScreenshot(MonitorLogScreenshot monitorLogScreenshot)
    {
        return monitorLogScreenshotMapper.updateMonitorLogScreenshot(monitorLogScreenshot);
    }

    /**
     * 批量删除截图日志
     * 
     * @param logIds 需要删除的截图日志ID
     * @return 结果
     */
    @Override
    public int deleteMonitorLogScreenshotByLogIds(Long[] logIds)
    {
        return monitorLogScreenshotMapper.deleteMonitorLogScreenshotByLogIds(logIds);
    }

    /**
     * 删除截图日志信息
     * 
     * @param logId 截图日志ID
     * @return 结果
     */
    @Override
    public int deleteMonitorLogScreenshotByLogId(Long logId)
    {
        return monitorLogScreenshotMapper.deleteMonitorLogScreenshotByLogId(logId);
    }
}
