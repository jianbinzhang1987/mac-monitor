package com.ruoyi.library.mapper;

import java.util.List;
import com.ruoyi.common.annotation.DataScope;
import com.ruoyi.library.domain.SysBook;
import com.ruoyi.library.domain.SysBookChapter;

/**
 * 图书管理Mapper接口
 *
 * @author ruoyi
 */
public interface SysBookMapper
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
    @DataScope(deptAlias = "d")
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
     * 删除图书管理
     *
     * @param bookId 图书管理主键
     * @return 结果
     */
    public int deleteBookByBookId(Long bookId);

    /**
     * 批量删除图书管理
     *
     * @param bookIds 需要删除的数据主键集合
     * @return 结果
     */
    public int deleteBookByBookIds(Long[] bookIds);

    /**
     * 批量删除图书章节
     *
     * @param bookIds 需要删除的数据主键集合
     * @return 结果
     */
    public int deleteSysBookChapterByBookIds(Long[] bookIds);

    /**
     * 通过图书管理主键删除图书章节信息
     *
     * @param bookId 图书管理ID
     * @return 结果
     */
    public int deleteSysBookChapterByBookId(Long bookId);

    /**
     * 批量新增图书章节
     *
     * @param sysBookChapterList 图书章节列表
     * @return 结果
     */
    public int batchSysBookChapter(List<SysBookChapter> sysBookChapterList);

    /**
     * 查询子元素
     *
     * @param bookId 图书ID
     * @return 子元素列表
     */
    public List<SysBook> selectBookChildren(Long bookId);

    /**
     * 批量修改子元素关系
     *
     * @param books 子元素列表
     * @return 结果
     */
    public int updateBookChildren(List<SysBook> books);

    /**
     * 查询是否存在子节点
     *
     * @param bookId 图书ID
     * @return 结果
     */
    public int countChildBookByBookId(Long bookId);
}
