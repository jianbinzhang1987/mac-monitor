package com.ruoyi.monitor.service.impl;

import java.util.List;
import com.ruoyi.common.utils.DateUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import com.ruoyi.monitor.mapper.MonitorPolicyMapper;
import com.ruoyi.monitor.domain.MonitorPolicy;
import com.ruoyi.monitor.service.IMonitorPolicyService;

/**
 * 监控策略Service业务层处理
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
@Service
public class MonitorPolicyServiceImpl implements IMonitorPolicyService {
    @Autowired
    private MonitorPolicyMapper monitorPolicyMapper;

    /**
     * 查询监控策略
     * 
     * @param policyId 监控策略ID
     * @return 监控策略
     */
    @Override
    public MonitorPolicy selectMonitorPolicyByPolicyId(Long policyId) {
        return monitorPolicyMapper.selectMonitorPolicyByPolicyId(policyId);
    }

    /**
     * 查询监控策略列表
     * 
     * @param monitorPolicy 监控策略
     * @return 监控策略
     */
    @Override
    public List<MonitorPolicy> selectMonitorPolicyList(MonitorPolicy monitorPolicy) {
        return monitorPolicyMapper.selectMonitorPolicyList(monitorPolicy);
    }

    /**
     * 新增监控策略
     * 
     * @param monitorPolicy 监控策略
     * @return 结果
     */
    @Override
    public int insertMonitorPolicy(MonitorPolicy monitorPolicy) {
        monitorPolicy.setCreateTime(DateUtils.getNowDate());
        if (monitorPolicy.getVersion() == null || monitorPolicy.getVersion().isEmpty()) {
            monitorPolicy.setVersion(String.valueOf(System.currentTimeMillis()));
        }
        if ("1".equals(monitorPolicy.getIsDefault())) {
            monitorPolicyMapper.resetDefaultPolicy();
        }
        return monitorPolicyMapper.insertMonitorPolicy(monitorPolicy);
    }

    /**
     * 修改监控策略
     * 
     * @param monitorPolicy 监控策略
     * @return 结果
     */
    @Override
    public int updateMonitorPolicy(MonitorPolicy monitorPolicy) {
        monitorPolicy.setUpdateTime(DateUtils.getNowDate());
        if (monitorPolicy.getVersion() == null || monitorPolicy.getVersion().isEmpty()) {
            monitorPolicy.setVersion(String.valueOf(System.currentTimeMillis()));
        }
        if ("1".equals(monitorPolicy.getIsDefault())) {
            monitorPolicyMapper.resetDefaultPolicy();
        }
        return monitorPolicyMapper.updateMonitorPolicy(monitorPolicy);
    }

    /**
     * 批量删除监控策略
     * 
     * @param policyIds 需要删除的监控策略ID
     * @return 结果
     */
    @Override
    public int deleteMonitorPolicyByPolicyIds(Long[] policyIds) {
        return monitorPolicyMapper.deleteMonitorPolicyByPolicyIds(policyIds);
    }

    /**
     * 删除监控策略信息
     * 
     * @param policyId 监控策略ID
     * @return 结果
     */
    @Override
    public int deleteMonitorPolicyByPolicyId(Long policyId) {
        return monitorPolicyMapper.deleteMonitorPolicyByPolicyId(policyId);
    }

    /**
     * 获取默认策略
     */
    @Override
    public MonitorPolicy selectDefaultPolicy() {
        return monitorPolicyMapper.selectDefaultPolicy();
    }

    /**
     * 快速添加策略规则
     */
    @Override
    public int quickAddRule(Long policyId, String type, String value) {
        MonitorPolicy policy = monitorPolicyMapper.selectMonitorPolicyByPolicyId(policyId);
        if (policy == null) {
            return 0;
        }

        String agentSettings = policy.getAgentSettings();
        com.alibaba.fastjson2.JSONObject settings;
        try {
            if (agentSettings == null || agentSettings.isEmpty()) {
                settings = new com.alibaba.fastjson2.JSONObject();
            } else {
                settings = com.alibaba.fastjson2.JSON.parseObject(agentSettings);
            }

            if ("PROCESS_BLOCK".equals(type)) {
                com.alibaba.fastjson2.JSONArray blockedProcesses = settings.getJSONArray("blockedProcesses");
                if (blockedProcesses == null) {
                    blockedProcesses = new com.alibaba.fastjson2.JSONArray();
                    settings.put("blockedProcesses", blockedProcesses);
                }
                if (!blockedProcesses.contains(value)) {
                    blockedProcesses.add(value);
                }
            } else if ("USB_DISABLE".equals(type)) {
                settings.put("allowUsb", false);
            }

            policy.setAgentSettings(settings.toJSONString());
            return updateMonitorPolicy(policy);
        } catch (Exception e) {
            return 0;
        }
    }
}
