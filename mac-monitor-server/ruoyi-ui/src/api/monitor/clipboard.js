import request from '@/utils/request'

// 查询剪贴板审计日志列表
export function listClipboard(query) {
  return request({
    url: '/monitor/log/clipboard/list',
    method: 'get',
    params: query
  })
}

// 查询剪贴板审计日志详细
export function getClipboard(logId) {
  return request({
    url: '/monitor/log/clipboard/' + logId,
    method: 'get'
  })
}

// 新增剪贴板审计日志
export function addClipboard(data) {
  return request({
    url: '/monitor/log/clipboard',
    method: 'post',
    data: data
  })
}

// 修改剪贴板审计日志
export function updateClipboard(data) {
  return request({
    url: '/monitor/log/clipboard',
    method: 'put',
    data: data
  })
}

// 删除剪贴板审计日志
export function delClipboard(logId) {
  return request({
    url: '/monitor/log/clipboard/' + logId,
    method: 'delete'
  })
}

// 导出剪贴板审计日志
export function exportClipboard(query) {
  return request({
    url: '/monitor/log/clipboard/export',
    method: 'get',
    params: query
  })
}
