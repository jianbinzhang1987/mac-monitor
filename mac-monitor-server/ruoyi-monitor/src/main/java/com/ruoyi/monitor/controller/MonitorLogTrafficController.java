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
import com.ruoyi.monitor.domain.MonitorLogTraffic;
import com.ruoyi.monitor.service.IMonitorLogTrafficService;
import com.ruoyi.common.utils.poi.ExcelUtil;
import com.ruoyi.common.core.page.TableDataInfo;

/**
 * 流量审计日志Controller
 *
 * @author ruoyi
 * @date 2026-01-15
 */
@RestController
@RequestMapping("/monitor/log/traffic")
public class MonitorLogTrafficController extends BaseController
{
    @Autowired
    private IMonitorLogTrafficService monitorLogTrafficService;

    /**
     * 查询流量审计日志列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:traffic:list')")
    @GetMapping("/list")
    public TableDataInfo list(MonitorLogTraffic monitorLogTraffic)
    {
        startPage();
        List<MonitorLogTraffic> list = monitorLogTrafficService.selectMonitorLogTrafficList(monitorLogTraffic);
        return getDataTable(list);
    }

    /**
     * 导出流量审计日志列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:traffic:export')")
    @Log(title = "流量审计日志", businessType = BusinessType.EXPORT)
    @GetMapping("/export")
    public void export(HttpServletResponse response, MonitorLogTraffic monitorLogTraffic)
    {
        List<MonitorLogTraffic> list = monitorLogTrafficService.selectMonitorLogTrafficList(monitorLogTraffic);
        ExcelUtil<MonitorLogTraffic> util = new ExcelUtil<MonitorLogTraffic>(MonitorLogTraffic.class);
        util.exportExcel(response, list, "流量审计日志数据");
    }

    /**
     * 获取流量审计日志详细信息
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:traffic:query')")
    @GetMapping(value = "/{logId}")
    public AjaxResult getInfo(@PathVariable("logId") Long logId)
    {
        return success(monitorLogTrafficService.selectMonitorLogTrafficByLogId(logId));
    }

    /**
     * 删除流量审计日志
     */
    @PreAuthorize("@ss.hasPermi('monitor:log:traffic:remove')")
    @Log(title = "流量审计日志", businessType = BusinessType.DELETE)
	@DeleteMapping("/{logIds}")
    public AjaxResult remove(@PathVariable Long[] logIds)
    {
        return toAjax(monitorLogTrafficService.deleteMonitorLogTrafficByLogIds(logIds));
    }
}
