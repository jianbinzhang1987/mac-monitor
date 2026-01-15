package com.ruoyi.monitor.mapper;

import java.util.List;
import com.ruoyi.monitor.domain.MonitorPolicy;

/**
 * 监控策略Mapper接口
 * 
 * @author ruoyi
 * @date 2026-01-15
 */
public interface MonitorPolicyMapper 
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
     * 删除监控策略
     * 
     * @param policyId 监控策略ID
     * @return 结果
     */
    public int deleteMonitorPolicyByPolicyId(Long policyId);

    /**
     * 批量删除监控策略
     * 
     * @param policyIds 需要删除的数据ID
     * @return 结果
     */
    public int deleteMonitorPolicyByPolicyIds(Long[] policyIds);
    
    /**
     * 获取默认策略
     * 
     * @return 默认策略
     */
    public MonitorPolicy selectDefaultPolicy();
    
    /**
     * 重置所有默认策略标志
     */
    public int resetDefaultPolicy();
}
