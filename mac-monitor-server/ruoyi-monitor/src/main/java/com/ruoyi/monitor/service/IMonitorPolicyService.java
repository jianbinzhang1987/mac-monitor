package com.ruoyi.monitor.service;

import java.util.List;
import com.ruoyi.monitor.domain.MonitorPolicy;

/**
 * 监控策略Service接口
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
public interface IMonitorPolicyService 
{
    /**
     * 查询监控策略
     * 
     * @param policyId 监控策略ID
     * @return 监控策略
     */
    public MonitorPolicy selectMonitorPolicyByPolicyId(Long policyId);

    /**
     * 查询监控策略列表
     * 
     * @param monitorPolicy 监控策略
     * @return 监控策略集合
     */
    public List<MonitorPolicy> selectMonitorPolicyList(MonitorPolicy monitorPolicy);

    /**
     * 新增监控策略
     * 
     * @param monitorPolicy 监控策略
     * @return 结果
     */
    public int insertMonitorPolicy(MonitorPolicy monitorPolicy);

    /**
     * 修改监控策略
     * 
     * @param monitorPolicy 监控策略
     * @return 结果
     */
    public int updateMonitorPolicy(MonitorPolicy monitorPolicy);

    /**
     * 批量删除监控策略
     * 
     * @param policyIds 需要删除的监控策略ID
     * @return 结果
     */
    public int deleteMonitorPolicyByPolicyIds(Long[] policyIds);

    /**
     * 删除监控策略信息
     * 
     * @param policyId 监控策略ID
     * @return 结果
     */
    public int deleteMonitorPolicyByPolicyId(Long policyId);
    
    /**
     * 获取默认策略
     *
     * @return 默认策略
     */
    public MonitorPolicy selectDefaultPolicy();

    /**
     * 快速添加策略规则
     * @param policyId 策略ID
     * @param type 规则类型 (PROCESS_BLOCK / USB_DISABLE)
     * @param value 规则内容 (进程名等)
     * @return 结果
     */
    public int quickAddRule(Long policyId, String type, String value);
}
