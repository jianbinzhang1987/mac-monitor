import request from '@/utils/request'

// 查询监控策略列表
export function listPolicy(query) {
    return request({
        url: '/monitor/policy/list',
        method: 'get',
        params: query
    })
}

// 查询监控策略详细
export function getPolicy(policyId) {
    return request({
        url: '/monitor/policy/' + policyId,
        method: 'get'
    })
}

// 新增监控策略
export function addPolicy(data) {
    return request({
        url: '/monitor/policy',
        method: 'post',
        data: data
    })
}

// 修改监控策略
export function updatePolicy(data) {
    return request({
        url: '/monitor/policy',
        method: 'put',
        data: data
    })
}

// 删除监控策略
export function delPolicy(policyId) {
    return request({
        url: '/monitor/policy/' + policyId,
        method: 'delete'
    })
}

// 快速联动：添加策略规则
export function quickAddRule(data) {
    return request({
        url: '/monitor/policy/quickAddRule',
        method: 'put',
        data: data
    })
}
