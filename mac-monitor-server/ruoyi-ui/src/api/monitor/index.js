import request from '@/utils/request'

// 获取首页统计数据
export function getIndexTotal() {
  return request({
    url: '/monitor/index/total',
    method: 'get'
  })
}
