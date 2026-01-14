package com.ruoyi.asset.controller;

import java.util.List;
import javax.servlet.http.HttpServletResponse;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import com.ruoyi.common.annotation.Log;
import com.ruoyi.common.core.controller.BaseController;
import com.ruoyi.common.core.domain.AjaxResult;
import com.ruoyi.common.enums.BusinessType;
import com.ruoyi.asset.domain.BizAsset;
import com.ruoyi.asset.service.IBizAssetService;
import com.ruoyi.common.utils.poi.ExcelUtil;
import com.ruoyi.common.core.page.TableDataInfo;
import io.swagger.annotations.Api;
import io.swagger.annotations.ApiOperation;

/**
 * 资产信息Controller
 *
 * @author ruoyi
 */
@Api(tags = "资产信息")
@RestController
@RequestMapping("/asset/info")
public class BizAssetController extends BaseController
{
    @Autowired
    private IBizAssetService bizAssetService;

    /**
     * 查询资产信息列表
     */
    @ApiOperation("查询资产信息列表")
    @PreAuthorize("@ss.hasPermi('asset:info:list')")
    @GetMapping("/list")
    public TableDataInfo list(BizAsset bizAsset)
    {
        startPage();
        List<BizAsset> list = bizAssetService.selectBizAssetList(bizAsset);
        return getDataTable(list);
    }

    /**
     * 导出资产信息列表
     */
    @ApiOperation("导出资产信息列表")
    @PreAuthorize("@ss.hasPermi('asset:info:export')")
    @Log(title = "资产信息", businessType = BusinessType.EXPORT)
    @PostMapping("/export")
    public void export(HttpServletResponse response, BizAsset bizAsset)
    {
        List<BizAsset> list = bizAssetService.selectBizAssetList(bizAsset);
        ExcelUtil<BizAsset> util = new ExcelUtil<BizAsset>(BizAsset.class);
        util.exportExcel(response, list, "资产信息数据");
    }

    /**
     * 获取资产信息详细信息
     */
    @ApiOperation("获取资产信息详细信息")
    @PreAuthorize("@ss.hasPermi('asset:info:query')")
    @GetMapping(value = "/{assetId}")
    public AjaxResult getInfo(@PathVariable("assetId") Long assetId)
    {
        return AjaxResult.success(bizAssetService.selectBizAssetByAssetId(assetId));
    }

    /**
     * 新增资产信息
     */
    @ApiOperation("新增资产信息")
    @PreAuthorize("@ss.hasPermi('asset:info:add')")
    @Log(title = "资产信息", businessType = BusinessType.INSERT)
    @PostMapping
    public AjaxResult add(@RequestBody BizAsset bizAsset)
    {
        bizAsset.setCreateBy(getUsername());
        return toAjax(bizAssetService.insertBizAsset(bizAsset));
    }

    /**
     * 修改资产信息
     */
    @ApiOperation("修改资产信息")
    @PreAuthorize("@ss.hasPermi('asset:info:edit')")
    @Log(title = "资产信息", businessType = BusinessType.UPDATE)
    @PutMapping
    public AjaxResult edit(@RequestBody BizAsset bizAsset)
    {
        bizAsset.setUpdateBy(getUsername());
        return toAjax(bizAssetService.updateBizAsset(bizAsset));
    }

    /**
     * 删除资产信息
     */
    @ApiOperation("删除资产信息")
    @PreAuthorize("@ss.hasPermi('asset:info:remove')")
    @Log(title = "资产信息", businessType = BusinessType.DELETE)
	@DeleteMapping("/{assetIds}")
    public AjaxResult remove(@PathVariable Long[] assetIds)
    {
        return toAjax(bizAssetService.deleteBizAssetByAssetIds(assetIds));
    }
}
