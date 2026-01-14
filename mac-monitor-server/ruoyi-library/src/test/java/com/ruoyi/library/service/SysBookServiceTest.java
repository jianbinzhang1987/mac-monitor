package com.ruoyi.library.service;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.Mockito.*;

import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.List;

import javax.validation.Validator;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.junit.jupiter.MockitoExtension;

import com.ruoyi.library.domain.SysBook;
import com.ruoyi.library.domain.SysBookChapter;
import com.ruoyi.library.mapper.SysBookMapper;
import com.ruoyi.library.service.impl.SysBookServiceImpl;

/**
 * 图书管理单元测试
 *
 * @author ruoyi
 */
@ExtendWith(MockitoExtension.class)
public class SysBookServiceTest {

    @InjectMocks
    private SysBookServiceImpl bookService;

    @Mock
    private SysBookMapper bookMapper;

    @Mock
    private Validator validator;

    private SysBook sampleBook;

    @BeforeEach
    public void setUp() {
        sampleBook = new SysBook();
        sampleBook.setBookId(100L);
        sampleBook.setBookName("测试图书");
        sampleBook.setAuthor("测试作者");
        sampleBook.setPrice(new BigDecimal("99.9"));
        sampleBook.setStatus("0");
    }

    /**
     * 测试新增图书及其章节 (主子表关联)
     */
    @Test
    public void testInsertBookWithChapters() {
        // 模拟数据
        List<SysBookChapter> chapters = new ArrayList<>();
        SysBookChapter chapter = new SysBookChapter();
        chapter.setChapterTitle("第一章");
        chapter.setOrderNum(1);
        chapters.add(chapter);
        sampleBook.setSysBookChapterList(chapters);

        // 模拟行为
        when(bookMapper.insertBook(any(SysBook.class))).thenReturn(1);

        // 执行
        int result = bookService.insertBook(sampleBook);

        // 验证
        assertEquals(1, result);
        verify(bookMapper, times(1)).insertBook(sampleBook);
        verify(bookMapper, times(1)).batchSysBookChapter(anyList());
    }

    /**
     * 测试查询图书列表
     */
    @Test
    public void testSelectBookList() {
        List<SysBook> list = new ArrayList<>();
        list.add(sampleBook);

        when(bookMapper.selectBookList(any(SysBook.class))).thenReturn(list);

        List<SysBook> result = bookService.selectBookList(new SysBook());

        assertNotNull(result);
        assertEquals(1, result.size());
        assertEquals("测试图书", result.get(0).getBookName());
    }

    /**
     * 测试删除图书
     */
    @Test
    public void testDeleteBook() {
        when(bookMapper.deleteSysBookChapterByBookId(100L)).thenReturn(1);
        when(bookMapper.deleteBookByBookId(100L)).thenReturn(1);

        int result = bookService.deleteBookByBookId(100L);

        assertEquals(1, result);
        verify(bookMapper).deleteSysBookChapterByBookId(100L);
        verify(bookMapper).deleteBookByBookId(100L);
    }
}
