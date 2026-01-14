import request from '@/utils/request'

// 查询图书管理列表
export function listBook(query) {
  return request({
    url: '/library/book/list',
    method: 'get',
    params: query
  })
}

// 查询图书管理详细
export function getBook(bookId) {
  return request({
    url: '/library/book/' + bookId,
    method: 'get'
  })
}

// 新增图书管理
export function addBook(data) {
  return request({
    url: '/library/book',
    method: 'post',
    data: data
  })
}

// 修改图书管理
export function updateBook(data) {
  return request({
    url: '/library/book',
    method: 'put',
    data: data
  })
}

// 删除图书管理
export function delBook(bookId) {
  return request({
    url: '/library/book/' + bookId,
    method: 'delete'
  })
}
