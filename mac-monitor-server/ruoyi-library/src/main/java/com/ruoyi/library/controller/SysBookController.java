package com.ruoyi.library.controller;

import java.util.List;
import javax.servlet.http.HttpServletResponse;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import com.ruoyi.common.annotation.Log;
import com.ruoyi.common.core.controller.BaseController;
import com.ruoyi.common.core.domain.AjaxResult;
import com.ruoyi.common.enums.BusinessType;
import com.ruoyi.library.domain.SysBook;
import com.ruoyi.library.service.ISysBookService;
import com.ruoyi.common.utils.poi.ExcelUtil;
import org.springframework.web.multipart.MultipartFile;
import io.swagger.annotations.Api;
import io.swagger.annotations.ApiOperation;

/**
 * 图書管理Controller
 *
 * @author ruoyi
 */
@Api(tags = "图书管理")
@RestController
@RequestMapping("/library/book")
public class SysBookController extends BaseController
{
    @Autowired
    private ISysBookService bookService;

    /**
     * 查询图书管理列表
     */
    @ApiOperation("查询图书管理列表")
    @PreAuthorize("@ss.hasPermi('library:book:list')")
    @GetMapping("/list")
    public AjaxResult list(SysBook book)
    {
        List<SysBook> list = bookService.selectBookList(book);
        return success(list);
    }

    /**
     * 导出图书管理列表
     */
    @ApiOperation("导出图书管理列表")
    @PreAuthorize("@ss.hasPermi('library:book:export')")
    @Log(title = "图书管理", businessType = BusinessType.EXPORT)
    @PostMapping("/export")
    public void export(HttpServletResponse response, SysBook book)
    {
        List<SysBook> list = bookService.selectBookList(book);
        ExcelUtil<SysBook> util = new ExcelUtil<SysBook>(SysBook.class);
        util.exportExcel(response, list, "图书管理数据");
    }

    /**
     * 导入图书管理列表
     */
    @ApiOperation("导入图书管理数据")
    @Log(title = "图书管理", businessType = BusinessType.IMPORT)
    @PreAuthorize("@ss.hasPermi('library:book:import')")
    @PostMapping("/importData")
    public AjaxResult importData(MultipartFile file, boolean updateSupport) throws Exception
    {
        ExcelUtil<SysBook> util = new ExcelUtil<SysBook>(SysBook.class);
        List<SysBook> bookList = util.importExcel(file.getInputStream());
        String operName = getUsername();
        String message = bookService.importBook(bookList, updateSupport, operName);
        return AjaxResult.success(message);
    }

    /**
     * 导出图书管理模板
     */
    @ApiOperation("获取导入模板")
    @PostMapping("/importTemplate")
    public void importTemplate(HttpServletResponse response)
    {
        ExcelUtil<SysBook> util = new ExcelUtil<SysBook>(SysBook.class);
        util.importTemplateExcel(response, "图书数据");
    }

    /**
     * 获取图书管理详细信息
     */
    @ApiOperation("获取图书管理详细信息")
    @PreAuthorize("@ss.hasPermi('library:book:query')")
    @GetMapping(value = "/{bookId}")
    public AjaxResult getInfo(@PathVariable("bookId") Long bookId)
    {
        return success(bookService.selectBookByBookId(bookId));
    }

    /**
     * 新增图书管理
     */
    @ApiOperation("新增图书管理")
    @PreAuthorize("@ss.hasPermi('library:book:add')")
    @Log(title = "图书管理", businessType = BusinessType.INSERT)
    @PostMapping
    public AjaxResult add(@RequestBody SysBook book)
    {
        book.setCreateBy(getUsername());
        return toAjax(bookService.insertBook(book));
    }

    /**
     * 修改图书管理
     */
    @ApiOperation("修改图书管理")
    @PreAuthorize("@ss.hasPermi('library:book:edit')")
    @Log(title = "图书管理", businessType = BusinessType.UPDATE)
    @PutMapping
    public AjaxResult edit(@RequestBody SysBook book)
    {
        book.setUpdateBy(getUsername());
        return toAjax(bookService.updateBook(book));
    }

    /**
     * 删除图书管理
     */
    @ApiOperation("删除图书管理")
    @PreAuthorize("@ss.hasPermi('library:book:remove')")
    @Log(title = "图书管理", businessType = BusinessType.DELETE)
	@DeleteMapping("/{bookId}")
    public AjaxResult remove(@PathVariable Long bookId)
    {
        if (bookService.hasChildByBookId(bookId))
        {
            return AjaxResult.error("存在下级图书,不允许删除");
        }
        return toAjax(bookService.deleteBookByBookId(bookId));
    }
}
