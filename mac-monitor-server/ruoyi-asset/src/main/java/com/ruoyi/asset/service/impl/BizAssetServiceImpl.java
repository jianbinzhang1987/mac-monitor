package com.ruoyi.asset.service.impl;

import java.util.List;
import com.ruoyi.common.utils.DateUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import com.ruoyi.asset.mapper.BizAssetMapper;
import com.ruoyi.asset.domain.BizAsset;
import com.ruoyi.asset.service.IBizAssetService;

/**
 * 资产信息Service业务层处理
 *
 * @author ruoyi
 */
@Service
public class BizAssetServiceImpl implements IBizAssetService
{
    @Autowired
    private BizAssetMapper bizAssetMapper;

    /**
     * 查询资产信息
     *
     * @param assetId 资产信息主键
     * @return 资产信息
     */
    @Override
    public BizAsset selectBizAssetByAssetId(Long assetId)
    {
        return bizAssetMapper.selectBizAssetByAssetId(assetId);
    }

    /**
     * 查询资产信息列表
     *
     * @param bizAsset 资产信息
     * @return 资产信息
     */
    @Override
    public List<BizAsset> selectBizAssetList(BizAsset bizAsset)
    {
        return bizAssetMapper.selectBizAssetList(bizAsset);
    }

    /**
     * 新增资产信息
     *
     * @param bizAsset 资产信息
     * @return 结果
     */
    @Override
    public int insertBizAsset(BizAsset bizAsset)
    {
        bizAsset.setCreateTime(DateUtils.getNowDate());
        return bizAssetMapper.insertBizAsset(bizAsset);
    }

    /**
     * 修改资产信息
     *
     * @param bizAsset 资产信息
     * @return 结果
     */
    @Override
    public int updateBizAsset(BizAsset bizAsset)
    {
        bizAsset.setUpdateTime(DateUtils.getNowDate());
        return bizAssetMapper.updateBizAsset(bizAsset);
    }

    /**
     * 批量删除资产信息
     *
     * @param assetIds 需要删除的资产信息主键
     * @return 结果
     */
    @Override
    public int deleteBizAssetByAssetIds(Long[] assetIds)
    {
        return bizAssetMapper.deleteBizAssetByAssetIds(assetIds);
    }

    /**
     * 删除资产信息信息
     *
     * @param assetId 资产信息主键
     * @return 结果
     */
    @Override
    public int deleteBizAssetByAssetId(Long assetId)
    {
        return bizAssetMapper.deleteBizAssetByAssetId(assetId);
    }
}
