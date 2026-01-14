package com.ruoyi.library.service.impl;

import java.util.List;
import com.ruoyi.common.utils.DateUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import com.ruoyi.library.mapper.SysBookMapper;
import com.ruoyi.library.domain.SysBook;
import com.ruoyi.library.domain.SysBookChapter;
import com.ruoyi.library.service.ISysBookService;
import com.ruoyi.common.exception.ServiceException;
import com.ruoyi.common.utils.StringUtils;
import com.ruoyi.common.utils.bean.BeanValidators;
import java.util.ArrayList;
import javax.validation.Validator;
import org.springframework.transaction.annotation.Transactional;

/**
 * 图书管理Service业务层处理
 *
 * @author ruoyi
 */
@Service
public class SysBookServiceImpl implements ISysBookService
{
    @Autowired
    private SysBookMapper sysBookMapper;

    @Autowired
    protected Validator validator;

    /**
     * 查询图书管理
     *
     * @param bookId 图书管理主键
     * @return 图书管理
     */
    @Override
    public SysBook selectBookByBookId(Long bookId)
    {
        return sysBookMapper.selectBookByBookId(bookId);
    }

    /**
     * 查询图书管理列表
     *
     * @param book 图书管理
     * @return 图书管理
     */
    @Override
    public List<SysBook> selectBookList(SysBook book)
    {
        return sysBookMapper.selectBookList(book);
    }

    /**
     * 新增图书管理
     *
     * @param book 图书管理
     * @return 结果
     */
    @Transactional
    @Override
    public int insertBook(SysBook book)
    {
        SysBook info = sysBookMapper.selectBookByBookId(book.getParentId());
        if (StringUtils.isNotNull(info))
        {
            book.setAncestors(info.getAncestors() + "," + book.getParentId());
        }
        else
        {
            book.setAncestors("0");
        }
        book.setCreateTime(DateUtils.getNowDate());
        int rows = sysBookMapper.insertBook(book);
        insertSysBookChapter(book);
        return rows;
    }

    /**
     * 修改图书管理
     *
     * @param book 图书管理
     * @return 结果
     */
    @Transactional
    @Override
    public int updateBook(SysBook book)
    {
        SysBook newParentEntity = sysBookMapper.selectBookByBookId(book.getParentId());
        SysBook oldEntity = sysBookMapper.selectBookByBookId(book.getBookId());
        if (StringUtils.isNotNull(newParentEntity) && StringUtils.isNotNull(oldEntity))
        {
            String newAncestors = newParentEntity.getAncestors() + "," + newParentEntity.getBookId();
            String oldAncestors = oldEntity.getAncestors();
            book.setAncestors(newAncestors);
            updateBookChildren(book.getBookId(), newAncestors, oldAncestors);
        }
        book.setUpdateTime(DateUtils.getNowDate());
        sysBookMapper.deleteSysBookChapterByBookId(book.getBookId());
        insertSysBookChapter(book);
        return sysBookMapper.updateBook(book);
    }

    /**
     * 修改子元素关系
     *
     * @param bookId 被修改的ID
     * @param newAncestors 新的父ID集合
     * @param oldAncestors 旧的父ID集合
     */
    public void updateBookChildren(Long bookId, String newAncestors, String oldAncestors)
    {
        List<SysBook> children = sysBookMapper.selectBookChildren(bookId);
        for (SysBook child : children)
        {
            child.setAncestors(child.getAncestors().replaceFirst(oldAncestors, newAncestors));
        }
        if (children.size() > 0)
        {
            sysBookMapper.updateBookChildren(children);
        }
    }

    /**
     * 批量删除图书管理
     *
     * @param bookIds 需要删除的数据主键
     * @return 结果
     */
    @Transactional
    @Override
    public int deleteBookByBookIds(Long[] bookIds)
    {
        sysBookMapper.deleteSysBookChapterByBookIds(bookIds);
        return sysBookMapper.deleteBookByBookIds(bookIds);
    }

    /**
     * 删除图书管理信息
     *
     * @param bookId 图书管理主键
     * @return 结果
     */
    @Transactional
    @Override
    public int deleteBookByBookId(Long bookId)
    {
        sysBookMapper.deleteSysBookChapterByBookId(bookId);
        return sysBookMapper.deleteBookByBookId(bookId);
    }

    /**
     * 是否存在图书子节点
     *
     * @param bookId 图书ID
     * @return 结果
     */
    @Override
    public boolean hasChildByBookId(Long bookId)
    {
        int result = sysBookMapper.countChildBookByBookId(bookId);
        return result > 0;
    }

    /**
     * 新增图书章节信息
     *
     * @param book 图书管理对象
     */
    public void insertSysBookChapter(SysBook book)
    {
        List<SysBookChapter> sysBookChapterList = book.getSysBookChapterList();
        Long bookId = book.getBookId();
        if (StringUtils.isNotNull(sysBookChapterList))
        {
            List<SysBookChapter> list = new ArrayList<SysBookChapter>();
            for (SysBookChapter sysBookChapter : sysBookChapterList)
            {
                sysBookChapter.setBookId(bookId);
                list.add(sysBookChapter);
            }
            if (list.size() > 0)
            {
                sysBookMapper.batchSysBookChapter(list);
            }
        }
    }

    /**
     * 导入图书数据
     *
     * @param bookList 图书数据列表
     * @param isUpdateSupport 是否更新支持，如果已存在，则进行更新数据
     * @param operName 操作用户
     * @return 结果
     */
    @Override
    public String importBook(List<SysBook> bookList, Boolean isUpdateSupport, String operName)
    {
        if (StringUtils.isNull(bookList) || bookList.size() == 0)
        {
            throw new ServiceException("导入图书数据不能为空！");
        }
        int successNum = 0;
        int failureNum = 0;
        StringBuilder successMsg = new StringBuilder();
        StringBuilder failureMsg = new StringBuilder();
        for (SysBook book : bookList)
        {
            try
            {
                // 验证是否存在这个图书 (根据图书名称判断，实际业务可能根据编码等)
                // 这里简单演示逻辑
                List<SysBook> existBooks = sysBookMapper.selectBookList(book);
                if (existBooks.isEmpty())
                {
                    BeanValidators.validateWithException(validator, book);
                    book.setCreateBy(operName);
                    this.insertBook(book);
                    successNum++;
                    successMsg.append("<br/>" + successNum + "、图书 " + book.getBookName() + " 导入成功");
                }
                else if (isUpdateSupport)
                {
                    BeanValidators.validateWithException(validator, book);
                    book.setUpdateBy(operName);
                    book.setBookId(existBooks.get(0).getBookId());
                    this.updateBook(book);
                    successNum++;
                    successMsg.append("<br/>" + successNum + "、图书 " + book.getBookName() + " 更新成功");
                }
                else
                {
                    failureNum++;
                    failureMsg.append("<br/>" + failureNum + "、图书 " + book.getBookName() + " 已存在");
                }
            }
            catch (Exception e)
            {
                failureNum++;
                String msg = "<br/>" + failureNum + "、图书 " + book.getBookName() + " 导入失败：";
                failureMsg.append(msg + e.getMessage());
            }
        }
        if (failureNum > 0)
        {
            failureMsg.insert(0, "很抱歉，导入失败！共 " + failureNum + " 条数据格式不正确，错误如下：");
            throw new ServiceException(failureMsg.toString());
        }
        else
        {
            successMsg.insert(0, "恭喜您，数据已全部导入成功！共 " + successNum + " 条，数据如下：");
        }
        return successMsg.toString();
    }
}
