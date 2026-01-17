package com.ruoyi.monitor.controller;

import java.util.List;
import javax.servlet.http.HttpServletResponse;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.access.prepost.PreAuthorize;
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
import com.ruoyi.monitor.domain.MonitorLogClipboard;
import com.ruoyi.monitor.service.IMonitorLogClipboardService;
import com.ruoyi.common.utils.poi.ExcelUtil;
import com.ruoyi.common.core.page.TableDataInfo;

/**
 * 剪贴板审计日志Controller
 *
 * @author ruoyi
 * @date 2026-01-17
 */
@RestController
@RequestMapping("/monitor/log/clipboard")
public class MonitorLogClipboardController extends BaseController
{
    @Autowired
    private IMonitorLogClipboardService monitorLogClipboardService;

    /**
     * 查询剪贴板审计日志列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:clipboard:list')")
    @GetMapping("/list")
    public TableDataInfo list(MonitorLogClipboard monitorLogClipboard)
    {
        startPage();
        List<MonitorLogClipboard> list = monitorLogClipboardService.selectMonitorLogClipboardList(monitorLogClipboard);
        return getDataTable(list);
    }

    /**
     * 导出剪贴板审计日志列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:clipboard:export')")
    @Log(title = "剪贴板审计日志", businessType = BusinessType.EXPORT)
    @GetMapping("/export")
    public void export(HttpServletResponse response, MonitorLogClipboard monitorLogClipboard)
    {
        List<MonitorLogClipboard> list = monitorLogClipboardService.selectMonitorLogClipboardList(monitorLogClipboard);
        ExcelUtil<MonitorLogClipboard> util = new ExcelUtil<MonitorLogClipboard>(MonitorLogClipboard.class);
        util.exportExcel(response, list, "剪贴板审计日志数据");
    }

    /**
     * 获取剪贴板审计日志详细信息
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:clipboard:query')")
    @GetMapping(value = "/{logId}")
    public AjaxResult getInfo(@PathVariable("logId") Long logId)
    {
        return success(monitorLogClipboardService.selectMonitorLogClipboardByLogId(logId));
    }

    /**
     * 新增剪贴板审计日志
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:clipboard:add')")
    @Log(title = "剪贴板审计日志", businessType = BusinessType.INSERT)
    @PostMapping
    public AjaxResult add(@RequestBody MonitorLogClipboard monitorLogClipboard)
    {
        return toAjax(monitorLogClipboardService.insertMonitorLogClipboard(monitorLogClipboard));
    }

    /**
     * 修改剪贴板审计日志
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:clipboard:edit')")
    @Log(title = "剪贴板审计日志", businessType = BusinessType.UPDATE)
    @PutMapping
    public AjaxResult edit(@RequestBody MonitorLogClipboard monitorLogClipboard)
    {
        return toAjax(monitorLogClipboardService.updateMonitorLogClipboard(monitorLogClipboard));
    }

    /**
     * 删除剪贴板审计日志
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:clipboard:remove')")
    @Log(title = "剪贴板审计日志", businessType = BusinessType.DELETE)
	@DeleteMapping("/{logIds}")
    public AjaxResult remove(@PathVariable Long[] logIds)
    {
        return toAjax(monitorLogClipboardService.deleteMonitorLogClipboardByLogIds(logIds));
    }
}
