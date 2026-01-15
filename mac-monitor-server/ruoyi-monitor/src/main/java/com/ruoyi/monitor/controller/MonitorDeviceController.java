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
import com.ruoyi.monitor.domain.MonitorDevice;
import com.ruoyi.monitor.service.IMonitorDeviceService;
import com.ruoyi.common.utils.poi.ExcelUtil;
import com.ruoyi.common.core.page.TableDataInfo;

/**
 * 监控设备Controller (管理后台)
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
@RestController
@RequestMapping("/monitor/device")
public class MonitorDeviceController extends BaseController
{
    @Autowired
    private IMonitorDeviceService monitorDeviceService;

    /**
     * 查询监控设备列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:device:list')")
    @GetMapping("/list")
    public TableDataInfo list(MonitorDevice monitorDevice)
    {
        startPage();
        List<MonitorDevice> list = monitorDeviceService.selectMonitorDeviceList(monitorDevice);
        return getDataTable(list);
    }

    /**
     * 导出监控设备列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:device:export')")
    // @Log(title = "监控设备", businessType = BusinessType.EXPORT)
    @PostMapping("/export")
    public void export(HttpServletResponse response, MonitorDevice monitorDevice)
    {
        List<MonitorDevice> list = monitorDeviceService.selectMonitorDeviceList(monitorDevice);
        ExcelUtil<MonitorDevice> util = new ExcelUtil<MonitorDevice>(MonitorDevice.class);
        util.exportExcel(response, list, "监控设备数据");
    }

    /**
     * 获取监控设备详细信息
     */
    @PreAuthorize("@ss.hasPermi('monitor:device:query')")
    @GetMapping(value = "/{deviceId}")
    public AjaxResult getInfo(@PathVariable("deviceId") Long deviceId)
    {
        return success(monitorDeviceService.selectMonitorDeviceByDeviceId(deviceId));
    }

    /**
     * 新增监控设备
     */
    @PreAuthorize("@ss.hasPermi('monitor:device:add')")
    // @Log(title = "监控设备", businessType = BusinessType.INSERT)
    @PostMapping
    public AjaxResult add(@RequestBody MonitorDevice monitorDevice)
    {
        return toAjax(monitorDeviceService.insertMonitorDevice(monitorDevice));
    }

    /**
     * 修改监控设备
     */
    @PreAuthorize("@ss.hasPermi('monitor:device:edit')")
    // @Log(title = "监控设备", businessType = BusinessType.UPDATE)
    @PutMapping
    public AjaxResult edit(@RequestBody MonitorDevice monitorDevice)
    {
        return toAjax(monitorDeviceService.updateMonitorDevice(monitorDevice));
    }

    /**
     * 批量修改设备关联策略
     */
    @PreAuthorize("@ss.hasPermi('monitor:device:edit')")
    @Log(title = "监控设备", businessType = BusinessType.UPDATE)
    @PutMapping("/updatePolicy")
    public AjaxResult updatePolicy(@RequestBody MonitorDevice monitorDevice)
    {
        return toAjax(monitorDeviceService.updateMonitorDevicePolicyBatch(monitorDevice.getDeviceIds(), monitorDevice.getPolicyId()));
    }

    /**
     * 删除监控设备
     */
    @PreAuthorize("@ss.hasPermi('monitor:device:remove')")
    // @Log(title = "监控设备", businessType = BusinessType.DELETE)
	@DeleteMapping("/{deviceIds}")
    public AjaxResult remove(@PathVariable Long[] deviceIds)
    {
        return toAjax(monitorDeviceService.deleteMonitorDeviceByDeviceIds(deviceIds));
    }
}
