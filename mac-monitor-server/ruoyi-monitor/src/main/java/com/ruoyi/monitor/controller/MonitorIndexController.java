package com.ruoyi.monitor.controller;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import com.ruoyi.common.core.domain.AjaxResult;
import com.ruoyi.monitor.service.IMonitorDeviceService;
import com.ruoyi.monitor.service.IMonitorLogBehaviorService;
import com.ruoyi.monitor.domain.MonitorDevice;
import com.ruoyi.monitor.domain.MonitorLogBehavior;

/**
 * 首页统计概览
 */
@RestController
@RequestMapping("/monitor/index")
public class MonitorIndexController {

    @Autowired
    private IMonitorDeviceService deviceService;

    @Autowired
    private IMonitorLogBehaviorService behaviorService;

    /**
     * 获取统计数据
     */
    @GetMapping("/total")
    public AjaxResult getTotal() {
        Map<String, Object> data = new HashMap<>();

        // 1. 设备统计
        MonitorDevice deviceQuery = new MonitorDevice();
        List<MonitorDevice> allDevices = deviceService.selectMonitorDeviceList(deviceQuery);
        long totalDevices = allDevices.size();
        long onlineDevices = allDevices.stream().filter(d -> "1".equals(d.getStatus())).count();

        Map<String, Long> deviceStats = new HashMap<>();
        deviceStats.put("total", totalDevices);
        deviceStats.put("online", onlineDevices);
        deviceStats.put("offline", totalDevices - onlineDevices);
        data.put("device", deviceStats);

        // 2. 行为风险统计 (今日)
        MonitorLogBehavior behaviorQuery = new MonitorLogBehavior();
        List<MonitorLogBehavior> allBehaviors = behaviorService.selectMonitorLogBehaviorList(behaviorQuery);

        long lowRisk = allBehaviors.stream().filter(b -> "0".equals(b.getRiskLevel())).count();
        long mediumRisk = allBehaviors.stream().filter(b -> "1".equals(b.getRiskLevel())).count();
        long highRisk = allBehaviors.stream().filter(b -> "2".equals(b.getRiskLevel())).count();

        Map<String, Long> riskStats = new HashMap<>();
        riskStats.put("low", lowRisk);
        riskStats.put("medium", mediumRisk);
        riskStats.put("high", highRisk);
        data.put("risk", riskStats);

        return AjaxResult.success(data);
    }
}
