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
import com.ruoyi.common.config.RuoYiConfig;
import com.ruoyi.common.utils.file.FileUploadUtils;
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
                ? String.valueOf(effectivePolicy.getVersion())
                : "0";

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
                ? String.valueOf(effectivePolicy.getVersion())
                : "0";

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
     * 获取策略配置 (黑名单等)
     * 对应 Rust 客户端: /api/v1/config/policy
     */
    @GetMapping("/config/policy")
    public AjaxResult getPolicyConfig() {
        // 简单实现：获取默认策略。如需针对设备，可扩展支持传参 serialNumber
        MonitorPolicy policy = monitorPolicyService.selectDefaultPolicy();
        if (policy == null) {
            // 如果没有默认策略，返回空配置以免客户端报错
            Map<String, Object> emptyData = new HashMap<>();
            emptyData.put("process_blacklist", new String[]{});
            emptyData.put("app_blacklist", new String[]{});
            return AjaxResult.success(emptyData);
        }

        Map<String, Object> data = new HashMap<>();
        try {
            // 解析 agentSettings 中的黑名单配置
            if (policy.getAgentSettings() != null) {
                com.alibaba.fastjson2.JSONObject settings = JSON.parseObject(policy.getAgentSettings());
                if (settings != null) {
                    data.put("process_blacklist", settings.getJSONArray("process_blacklist"));
                    data.put("app_blacklist", settings.getJSONArray("app_blacklist"));
                }
            }
        } catch (Exception e) {
            logger.error("解析策略配置失败", e);
        }

        // 确保字段存在
        data.putIfAbsent("process_blacklist", new String[]{});
        data.putIfAbsent("app_blacklist", new String[]{});

        return AjaxResult.success(data);
    }

    @Autowired
    private com.ruoyi.monitor.service.IMonitorLogTrafficService monitorLogTrafficService;

    @Autowired
    private com.ruoyi.monitor.service.IMonitorLogBehaviorService monitorLogBehaviorService;

    @Autowired
    private com.ruoyi.monitor.service.IMonitorLogScreenshotService monitorLogScreenshotService;

    @Autowired
    private com.ruoyi.monitor.service.IMonitorLogClipboardService monitorLogClipboardService;

    /**
     * 4. 流量/审计日志上报
     */
    @PostMapping("/log/audit")
    public AjaxResult uploadAuditLog(@RequestBody com.ruoyi.monitor.domain.MonitorLogTraffic log) {
        if (log.getSerialNumber() != null) {
            MonitorDevice device = monitorDeviceService.selectMonitorDeviceBySerialNumber(log.getSerialNumber());
            // 如果设备不存在，可以考虑先自动注册
            if (device == null) {
                device = new MonitorDevice();
                device.setSerialNumber(log.getSerialNumber());
                device.setDeviceName("Auto Registered");
                monitorDeviceService.registerOrUpdate(device);
                device = monitorDeviceService.selectMonitorDeviceBySerialNumber(log.getSerialNumber());
            }
        }
        monitorLogTrafficService.insertMonitorLogTraffic(log);
        return AjaxResult.success();
    }

    /**
     * 5. 行为日志上报
     */
    @PostMapping("/log/behavior")
    public AjaxResult uploadBehaviorLog(@RequestBody com.ruoyi.monitor.domain.MonitorLogBehavior log) {
        if (log.getSerialNumber() != null) {
            MonitorDevice device = monitorDeviceService.selectMonitorDeviceBySerialNumber(log.getSerialNumber());
            if (device == null) {
                device = new MonitorDevice();
                device.setSerialNumber(log.getSerialNumber());
                device.setDeviceName("Auto Registered");
                monitorDeviceService.registerOrUpdate(device);
            }
        }
        monitorLogBehaviorService.insertMonitorLogBehavior(log);
        return AjaxResult.success();
    }

    /**
     * 6. 截图日志上报
     */
    @PostMapping("/log/screenshot")
    public AjaxResult uploadScreenshotLog(@RequestBody com.ruoyi.monitor.domain.MonitorLogScreenshot log) {
        if (log.getSerialNumber() != null) {
            MonitorDevice device = monitorDeviceService.selectMonitorDeviceBySerialNumber(log.getSerialNumber());
            if (device != null) {
                log.setDeviceId(device.getDeviceId());
            } else {
                // 自动注册设备
                MonitorDevice newDevice = new MonitorDevice();
                newDevice.setSerialNumber(log.getSerialNumber());
                newDevice.setDeviceName(log.getHostId() != null ? log.getHostId() : "Auto Registered");
                newDevice.setRegisteredIp(log.getIp());
                monitorDeviceService.registerOrUpdate(newDevice);

                // 重新获取以获取 ID
                device = monitorDeviceService.selectMonitorDeviceBySerialNumber(log.getSerialNumber());
                if (device != null) {
                    log.setDeviceId(device.getDeviceId());
                }
            }
        }
        monitorLogScreenshotService.insertMonitorLogScreenshot(log);
        return AjaxResult.success();
    }

    /**
     * 7. 截图文件上传
     */
    @Anonymous
    @PostMapping("/upload/screenshot")
    public AjaxResult uploadScreenshot(MultipartFile file)
    {
        try
        {
            // 上传文件路径
            String filePath = RuoYiConfig.getUploadPath();
            // 上传并返回新文件名称 (返回格式如 /profile/upload/2026/01/15/xxx.jpg)
            String fileName = FileUploadUtils.upload(filePath, file);

            AjaxResult ajax = AjaxResult.success();
            ajax.put("url", fileName);
            ajax.put("fileName", fileName);
            return ajax;
        }
        catch (Exception e)
        {
            return AjaxResult.error(e.getMessage());
        }
    }

    /**
     * 8. 剪贴板日志上报
     */
    @PostMapping("/log/clipboard")
    public AjaxResult uploadClipboardLog(@RequestBody com.ruoyi.monitor.domain.MonitorLogClipboard log) {
        if (log.getSerialNumber() != null) {
            MonitorDevice device = monitorDeviceService.selectMonitorDeviceBySerialNumber(log.getSerialNumber());
            if (device != null) {
                log.setDeviceId(device.getDeviceId());
            } else {
                // 自动注册设备
                MonitorDevice newDevice = new MonitorDevice();
                newDevice.setSerialNumber(log.getSerialNumber());
                newDevice.setDeviceName(log.getHostId() != null ? log.getHostId() : "Auto Registered");
                newDevice.setRegisteredIp(log.getIp());
                monitorDeviceService.registerOrUpdate(newDevice);

                // 重新获取以获取 ID
                device = monitorDeviceService.selectMonitorDeviceBySerialNumber(log.getSerialNumber());
                if (device != null) {
                    log.setDeviceId(device.getDeviceId());
                }
            }
        }
        monitorLogClipboardService.insertMonitorLogClipboard(log);
        return AjaxResult.success();
    }
}
