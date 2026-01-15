package com.ruoyi.monitor.controller;

import com.ruoyi.common.annotation.Anonymous;
import com.ruoyi.common.core.controller.BaseController;
import com.ruoyi.common.core.domain.AjaxResult;
import com.ruoyi.common.utils.ServletUtils;
import com.ruoyi.common.utils.ip.IpUtils;
import com.ruoyi.monitor.domain.MonitorDevice;
import com.ruoyi.monitor.domain.MonitorPolicy;
import com.ruoyi.monitor.service.IMonitorDeviceService;
import com.ruoyi.monitor.service.IMonitorPolicyService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;
import com.alibaba.fastjson2.JSON;

import java.util.HashMap;
import java.util.Map;

/**
 * Mac 客户端 API 接口
 * 
 * @author ruoyi
 */
@Anonymous
@RestController
@RequestMapping("/api/v1")
public class ApiMonitorController extends BaseController {

    @Autowired
    private IMonitorDeviceService monitorDeviceService;
    
    @Autowired
    private IMonitorPolicyService monitorPolicyService;

    /**
     * 获取设备对应的生效策略
     */
    private MonitorPolicy getEffectivePolicy(String serialNumber) {
        MonitorPolicy policy = null;
        if (serialNumber != null) {
            MonitorDevice device = monitorDeviceService.selectMonitorDeviceBySerialNumber(serialNumber);
            if (device != null && device.getPolicyId() != null) {
                policy = monitorPolicyService.selectMonitorPolicyByPolicyId(device.getPolicyId());
            }
        }
        if (policy == null) {
            policy = monitorPolicyService.selectDefaultPolicy();
        }
        return policy;
    }

    /**
     * 1. 客户端登录/注册
     */
    @PostMapping("/login")
    public AjaxResult login(@RequestBody MonitorDevice device) {
        // 获取客户端IP
        device.setRegisteredIp(IpUtils.getIpAddr(ServletUtils.getRequest()));

        String token = monitorDeviceService.registerOrUpdate(device);

        // 获取该设备对应的生效策略版本
        MonitorPolicy effectivePolicy = getEffectivePolicy(device.getSerialNumber());
        String policyVersion = (effectivePolicy != null && effectivePolicy.getVersion() != null)
                             ? String.valueOf(effectivePolicy.getVersion()) : "0";

        Map<String, Object> data = new HashMap<>();
        data.put("token", token);
        // 返回服务端最新的策略版本，客户端对比后决定是否立即请求 /config
        data.put("policyVersion", policyVersion);

        return AjaxResult.success("登录成功", data);
    }

    /**
     * 2. 心跳保活
     * 客户端每 30-60s 发送一次
     */
    @PostMapping("/heartbeat")
    public AjaxResult heartbeat(@RequestBody MonitorDevice device) {
        MonitorDevice exist = monitorDeviceService.selectMonitorDeviceBySerialNumber(device.getSerialNumber());
        if (exist == null) {
            return AjaxResult.error(401, "设备未注册");
        }

        // 查询该设备对应的生效策略
        MonitorPolicy effectivePolicy = getEffectivePolicy(device.getSerialNumber());
        String serverPolicyVersion = (effectivePolicy != null && effectivePolicy.getVersion() != null)
                                   ? String.valueOf(effectivePolicy.getVersion()) : "0";

        // 更新最后心跳时间
        exist.setLastHeartbeat(new java.util.Date());
        exist.setStatus("1");
        // 更新设备当前汇报的版本
        exist.setPolicyVersion(device.getPolicyVersion());
        monitorDeviceService.updateMonitorDevice(exist);

        // 检查配置是否需要更新
        // 如果客户端传来的版本 与 服务端最新版本 不一致，则通知更新
        boolean needUpdate = false;
        if (device.getPolicyVersion() != null && !serverPolicyVersion.equals(device.getPolicyVersion())) {
            needUpdate = true;
        }

        Map<String, Object> data = new HashMap<>();
        data.put("serverTime", System.currentTimeMillis());
        data.put("needUpdate", needUpdate);
        data.put("latestVersion", serverPolicyVersion);

        return AjaxResult.success(data);
    }

    /**
     * 3. 获取配置
     */
    @GetMapping("/config")
    public AjaxResult getConfig(@RequestParam(required = false) String serialNumber) {
        // 根据设备序列号获取生效策略（优先关联策略，次之默认策略）
        MonitorPolicy policy = getEffectivePolicy(serialNumber);

        if (policy == null) {
            return error("无可用策略配置");
        }

        Map<String, Object> data = new HashMap<>();
        data.put("version", String.valueOf(policy.getVersion())); // 转String防止精度丢失
        data.put("policyName", policy.getPolicyName());

        try {
            // 尝试将 JSON 字符串转为 Object 返回，方便前端/客户端处理
            if (policy.getTrafficRules() != null) {
                data.put("trafficRules", JSON.parseObject(policy.getTrafficRules()));
            }
            if (policy.getScreenshotRules() != null) {
                data.put("screenshotRules", JSON.parseObject(policy.getScreenshotRules()));
            }
            if (policy.getAgentSettings() != null) {
                data.put("agentSettings", JSON.parseObject(policy.getAgentSettings()));
            }
        } catch (Exception e) {
            // 解析失败则返回原始字符串
            data.put("trafficRules", policy.getTrafficRules());
            data.put("screenshotRules", policy.getScreenshotRules());
            data.put("agentSettings", policy.getAgentSettings());
        }

        return AjaxResult.success(data);
    }

    /**
     * 4. 日志上报
     */
    @PostMapping("/log/upload")
    public AjaxResult uploadLog(@RequestParam(required = false) MultipartFile file, 
                                @RequestParam(required = false) String logJson) {
        // 如果有图片文件 (截图日志)
        if (file != null && !file.isEmpty()) {
            // TODO: 保存文件到 OSS 或本地磁盘
            // String filePath = FileUploadUtils.upload(file);
        }
        
        // 解析 logJson 并入库 (monitor_log_traffic / monitor_log_screenshot)
        // LogService.saveLog(logJson);
        
        return AjaxResult.success();
    }
}
