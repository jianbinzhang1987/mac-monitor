import request from '@/utils/request'

// 查询截图日志列表
export function listScreenshot(query) {
    return request({
        url: '/monitor/log/screenshot/list',
        method: 'get',
        params: query
    })
}

// 查询截图日志详细
export function getScreenshot(logId) {
    return request({
        url: '/monitor/log/screenshot/' + logId,
        method: 'get'
    })
}

// 新增截图日志
export function addScreenshot(data) {
    return request({
        url: '/monitor/log/screenshot',
        method: 'post',
        data: data
    })
}

// 修改截图日志
export function updateScreenshot(data) {
    return request({
        url: '/monitor/log/screenshot',
        method: 'put',
        data: data
    })
}

// 删除截图日志
export function delScreenshot(logId) {
    return request({
        url: '/monitor/log/screenshot/' + logId,
        method: 'delete'
    })
}
