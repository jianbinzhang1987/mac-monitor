package com.ruoyi.monitor.controller;

import java.util.List;
import javax.servlet.http.HttpServletResponse;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import com.ruoyi.common.annotation.Log;
import com.ruoyi.common.core.controller.BaseController;
import com.ruoyi.common.core.domain.AjaxResult;
import com.ruoyi.common.enums.BusinessType;
import com.ruoyi.monitor.domain.MonitorLogBehavior;
import com.ruoyi.monitor.service.IMonitorLogBehaviorService;
import com.ruoyi.common.utils.poi.ExcelUtil;
import com.ruoyi.common.core.page.TableDataInfo;

/**
 * 行为审计日志Controller
 *
 * @author ruoyi
 * @date 2026-01-15
 */
@RestController
@RequestMapping("/monitor/log/behavior")
public class MonitorLogBehaviorController extends BaseController
{
    @Autowired
    private IMonitorLogBehaviorService monitorLogBehaviorService;

    /**
     * 查询行为审计日志列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:behavior:list')")
    @GetMapping("/list")
    public TableDataInfo list(MonitorLogBehavior monitorLogBehavior)
    {
        startPage();
        List<MonitorLogBehavior> list = monitorLogBehaviorService.selectMonitorLogBehaviorList(monitorLogBehavior);
        return getDataTable(list);
    }

    /**
     * 导出行为审计日志列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:behavior:export')")
    @Log(title = "行为审计日志", businessType = BusinessType.EXPORT)
    @GetMapping("/export")
    public void export(HttpServletResponse response, MonitorLogBehavior monitorLogBehavior)
    {
        List<MonitorLogBehavior> list = monitorLogBehaviorService.selectMonitorLogBehaviorList(monitorLogBehavior);
        ExcelUtil<MonitorLogBehavior> util = new ExcelUtil<MonitorLogBehavior>(MonitorLogBehavior.class);
        util.exportExcel(response, list, "行为审计日志数据");
    }

    /**
     * 获取行为审计日志详细信息
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:behavior:query')")
    @GetMapping(value = "/{logId}")
    public AjaxResult getInfo(@PathVariable("logId") Long logId)
    {
        return success(monitorLogBehaviorService.selectMonitorLogBehaviorByLogId(logId));
    }

    /**
     * 删除行为审计日志
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:behavior:remove')")
    @Log(title = "行为审计日志", businessType = BusinessType.DELETE)
	@DeleteMapping("/{logIds}")
    public AjaxResult remove(@PathVariable Long[] logIds)
    {
        return toAjax(monitorLogBehaviorService.deleteMonitorLogBehaviorByLogIds(logIds));
    }
}
