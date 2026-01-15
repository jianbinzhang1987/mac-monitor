import request from '@/utils/request'

// 查询流量审计日志列表
export function listTraffic(query) {
  return request({
    url: '/monitor/log/traffic/list',
    method: 'get',
    params: query
  })
}

// 查询流量审计日志详细
export function getTraffic(logId) {
  return request({
    url: '/monitor/log/traffic/' + logId,
    method: 'get'
  })
}

// 删除流量审计日志
export function delTraffic(logId) {
  return request({
    url: '/monitor/log/traffic/' + logId,
    method: 'delete'
  })
}
