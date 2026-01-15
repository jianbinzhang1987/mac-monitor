package com.ruoyi.monitor.controller;

import java.util.List;
import javax.servlet.http.HttpServletResponse;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import com.ruoyi.common.annotation.Log;
import com.ruoyi.common.core.controller.BaseController;
import com.ruoyi.common.core.domain.AjaxResult;
import com.ruoyi.common.enums.BusinessType;
import com.ruoyi.monitor.domain.MonitorLogScreenshot;
import com.ruoyi.monitor.service.IMonitorLogScreenshotService;
import com.ruoyi.common.utils.poi.ExcelUtil;
import com.ruoyi.common.core.page.TableDataInfo;

/**
 * 截图日志Controller (管理后台)
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
@RestController
@RequestMapping("/monitor/log/screenshot")
public class MonitorLogScreenshotController extends BaseController
{
    @Autowired
    private IMonitorLogScreenshotService monitorLogScreenshotService;

    /**
     * 查询截图日志列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:screenshot:list')")
    @GetMapping("/list")
    public TableDataInfo list(MonitorLogScreenshot monitorLogScreenshot)
    {
        startPage();
        List<MonitorLogScreenshot> list = monitorLogScreenshotService.selectMonitorLogScreenshotList(monitorLogScreenshot);
        return getDataTable(list);
    }

    /**
     * 导出截图日志列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:screenshot:export')")
    // @Log(title = "截图日志", businessType = BusinessType.EXPORT)
    @PostMapping("/export")
    public void export(HttpServletResponse response, MonitorLogScreenshot monitorLogScreenshot)
    {
        List<MonitorLogScreenshot> list = monitorLogScreenshotService.selectMonitorLogScreenshotList(monitorLogScreenshot);
        ExcelUtil<MonitorLogScreenshot> util = new ExcelUtil<MonitorLogScreenshot>(MonitorLogScreenshot.class);
        util.exportExcel(response, list, "截图日志数据");
    }

    /**
     * 获取截图日志详细信息
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:screenshot:query')")
    @GetMapping(value = "/{logId}")
    public AjaxResult getInfo(@PathVariable("logId") Long logId)
    {
        return success(monitorLogScreenshotService.selectMonitorLogScreenshotByLogId(logId));
    }

    /**
     * 新增截图日志 (通常由Client上传，此处保留手动添加接口用于测试)
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:screenshot:add')")
    // @Log(title = "截图日志", businessType = BusinessType.INSERT)
    @PostMapping
    public AjaxResult add(@RequestBody MonitorLogScreenshot monitorLogScreenshot)
    {
        return toAjax(monitorLogScreenshotService.insertMonitorLogScreenshot(monitorLogScreenshot));
    }

    /**
     * 修改截图日志 (仅修改RiskLevel或Tags)
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:screenshot:edit')")
    // @Log(title = "截图日志", businessType = BusinessType.UPDATE)
    @PutMapping
    public AjaxResult edit(@RequestBody MonitorLogScreenshot monitorLogScreenshot)
    {
        return toAjax(monitorLogScreenshotService.updateMonitorLogScreenshot(monitorLogScreenshot));
    }

    /**
     * 删除截图日志
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:screenshot:remove')")
    // @Log(title = "截图日志", businessType = BusinessType.DELETE)
	@DeleteMapping("/{logIds}")
    public AjaxResult remove(@PathVariable Long[] logIds)
    {
        return toAjax(monitorLogScreenshotService.deleteMonitorLogScreenshotByLogIds(logIds));
    }
}
