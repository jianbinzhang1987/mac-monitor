package com.ruoyi.library.service;

import java.util.List;
import com.ruoyi.library.domain.SysBook;

/**
 * 图书管理Service接口
 *
 * @author ruoyi
 */
public interface ISysBookService
{
    /**
     * 查询图书管理
     *
     * @param bookId 图书管理主键
     * @return 图书管理
     */
    public SysBook selectBookByBookId(Long bookId);

    /**
     * 查询图书管理列表
     *
     * @param book 图书管理
     * @return 图书管理集合
     */
    public List<SysBook> selectBookList(SysBook book);

    /**
     * 新增图书管理
     *
     * @param book 图书管理
     * @return 结果
     */
    public int insertBook(SysBook book);

    /**
     * 修改图书管理
     *
     * @param book 图书管理
     * @return 结果
     */
    public int updateBook(SysBook book);

    /**
     * 批量删除图书管理
     *
     * @param bookIds 需要删除的数据主键集合
     * @return 结果
     */
    public int deleteBookByBookIds(Long[] bookIds);

    /**
     * 删除图书管理信息
     *
     * @param bookId 图书管理主键
     * @return 结果
     */
    public int deleteBookByBookId(Long bookId);

    /**
     * 是否存在图书子节点
     *
     * @param bookId 图书ID
     * @return 结果 true 存在 false 不存在
     */
    public boolean hasChildByBookId(Long bookId);

    /**
     * 导入图书数据
     *
     * @param bookList 图书数据列表
     * @param isUpdateSupport 是否更新支持，如果已存在，则进行更新数据
     * @param operName 操作用户
     * @return 结果
     */
    public String importBook(List<SysBook> bookList, Boolean isUpdateSupport, String operName);
}
