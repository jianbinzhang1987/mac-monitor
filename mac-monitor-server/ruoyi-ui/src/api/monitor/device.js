import request from '@/utils/request'

// 查询监控设备列表
export function listDevice(query) {
    return request({
        url: '/monitor/device/list',
        method: 'get',
        params: query
    })
}

// 查询监控设备详细
export function getDevice(deviceId) {
    return request({
        url: '/monitor/device/' + deviceId,
        method: 'get'
    })
}

// 新增监控设备
export function addDevice(data) {
    return request({
        url: '/monitor/device',
        method: 'post',
        data: data
    })
}

// 修改监控设备
export function updateDevice(data) {
    return request({
        url: '/monitor/device',
        method: 'put',
        data: data
    })
}

// 批量修改设备策略
export function updateDevicePolicy(deviceIds, policyId) {
    const data = {
        deviceIds,
        policyId
    }
    return request({
        url: '/monitor/device/updatePolicy',
        method: 'put',
        data: data
    })
}

// 删除监控设备
export function delDevice(deviceId) {
    return request({
        url: '/monitor/device/' + deviceId,
        method: 'delete'
    })
}
