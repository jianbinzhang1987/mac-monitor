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
import com.ruoyi.monitor.domain.MonitorPolicy;
import com.ruoyi.monitor.service.IMonitorPolicyService;
import com.ruoyi.common.utils.poi.ExcelUtil;
import com.ruoyi.common.core.page.TableDataInfo;

/**
 * 监控策略Controller (管理后台)
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
@RestController
@RequestMapping("/monitor/policy")
public class MonitorPolicyController extends BaseController
{
    @Autowired
    private IMonitorPolicyService monitorPolicyService;

    /**
     * 查询监控策略列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:policy:list')")
    @GetMapping("/list")
    public TableDataInfo list(MonitorPolicy monitorPolicy)
    {
        startPage();
        List<MonitorPolicy> list = monitorPolicyService.selectMonitorPolicyList(monitorPolicy);
        return getDataTable(list);
    }

    /**
     * 导出监控策略列表
     */
    @PreAuthorize("@ss.hasPermi('monitor:policy:export')")
    // @Log(title = "监控策略", businessType = BusinessType.EXPORT)
    @PostMapping("/export")
    public void export(HttpServletResponse response, MonitorPolicy monitorPolicy)
    {
        List<MonitorPolicy> list = monitorPolicyService.selectMonitorPolicyList(monitorPolicy);
        ExcelUtil<MonitorPolicy> util = new ExcelUtil<MonitorPolicy>(MonitorPolicy.class);
        util.exportExcel(response, list, "监控策略数据");
    }

    /**
     * 获取监控策略详细信息
     */
    @PreAuthorize("@ss.hasPermi('monitor:policy:query')")
    @GetMapping(value = "/{policyId}")
    public AjaxResult getInfo(@PathVariable("policyId") Long policyId)
    {
        return success(monitorPolicyService.selectMonitorPolicyByPolicyId(policyId));
    }

    /**
     * 新增监控策略
     */
    @PreAuthorize("@ss.hasPermi('monitor:policy:add')")
    // @Log(title = "监控策略", businessType = BusinessType.INSERT)
    @PostMapping
    public AjaxResult add(@RequestBody MonitorPolicy monitorPolicy)
    {
        return toAjax(monitorPolicyService.insertMonitorPolicy(monitorPolicy));
    }

    /**
     * 修改监控策略
     */
    @PreAuthorize("@ss.hasPermi('monitor:policy:edit')")
    // @Log(title = "监控策略", businessType = BusinessType.UPDATE)
    @PutMapping
    public AjaxResult edit(@RequestBody MonitorPolicy monitorPolicy)
    {
        return toAjax(monitorPolicyService.updateMonitorPolicy(monitorPolicy));
    }

    /**
     * 删除监控策略
     */
    @PreAuthorize("@ss.hasPermi('monitor:policy:remove')")
    // @Log(title = "监控策略", businessType = BusinessType.DELETE)
	@DeleteMapping("/{policyIds}")
    public AjaxResult remove(@PathVariable Long[] policyIds)
    {
        return toAjax(monitorPolicyService.deleteMonitorPolicyByPolicyIds(policyIds));
    }

    /**
     * 快速添加策略规则 (行为审计联动)
     */
    @PreAuthorize("@ss.hasPermi('monitor:policy:edit')")
    @Log(title = "监控策略", businessType = BusinessType.UPDATE)
    @PutMapping("/quickAddRule")
    public AjaxResult quickAddRule(@RequestBody MonitorPolicy monitorPolicy)
    {
        // 借用 MonitorPolicy 对象传递参数：policyId, type(备注存储), value(JSON字段存储)
        // 实际开发中建议定义专用 DTO，此处为保持简洁直接复用
        String type = monitorPolicy.getRemark();
        String value = monitorPolicy.getTrafficRules(); // 临时借用字段
        return toAjax(monitorPolicyService.quickAddRule(monitorPolicy.getPolicyId(), type, value));
    }
}
