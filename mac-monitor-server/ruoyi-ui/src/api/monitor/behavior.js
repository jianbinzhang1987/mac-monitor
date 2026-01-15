import request from '@/utils/request'

// 查询行为审计日志列表
export function listBehavior(query) {
  return request({
    url: '/monitor/log/behavior/list',
    method: 'get',
    params: query
  })
}

// 查询行为审计日志详细
export function getBehavior(logId) {
  return request({
    url: '/monitor/log/behavior/' + logId,
    method: 'get'
  })
}

// 删除行为审计日志
export function delBehavior(logId) {
  return request({
    url: '/monitor/log/behavior/' + logId,
    method: 'delete'
  })
}
