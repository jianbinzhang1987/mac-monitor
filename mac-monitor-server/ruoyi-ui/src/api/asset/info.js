import request from '@/utils/request'

// 查询资产信息列表
export function listAsset(query) {
  return request({
    url: '/asset/info/list',
    method: 'get',
    params: query
  })
}

// 查询资产信息详细
export function getAsset(assetId) {
  return request({
    url: '/asset/info/' + assetId,
    method: 'get'
  })
}

// 新增资产信息
export function addAsset(data) {
  return request({
    url: '/asset/info',
    method: 'post',
    data: data
  })
}

// 修改资产信息
export function updateAsset(data) {
  return request({
    url: '/asset/info',
    method: 'put',
    data: data
  })
}

// 删除资产信息
export function delAsset(assetId) {
  return request({
    url: '/asset/info/' + assetId,
    method: 'delete'
  })
}
