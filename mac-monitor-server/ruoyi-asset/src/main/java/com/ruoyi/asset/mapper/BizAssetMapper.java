package com.ruoyi.asset.mapper;

import java.util.List;
import com.ruoyi.asset.domain.BizAsset;

/**
 * 资产信息Mapper接口
 *
 * @author ruoyi
 */
public interface BizAssetMapper
{
    /**
     * 查询资产信息
     *
     * @param assetId 资产信息主键
     * @return 资产信息
     */
    public BizAsset selectBizAssetByAssetId(Long assetId);

    /**
     * 查询资产信息列表
     *
     * @param bizAsset 资产信息
     * @return 资产信息集合
     */
    public List<BizAsset> selectBizAssetList(BizAsset bizAsset);

    /**
     * 新增资产信息
     *
     * @param bizAsset 资产信息
     * @return 结果
     */
    public int insertBizAsset(BizAsset bizAsset);

    /**
     * 修改资产信息
     *
     * @param bizAsset 资产信息
     * @return 结果
     */
    public int updateBizAsset(BizAsset bizAsset);

    /**
     * 删除资产信息
     *
     * @param assetId 资产信息主键
     * @return 结果
     */
    public int deleteBizAssetByAssetId(Long assetId);

    /**
     * 批量删除资产信息
     *
     * @param assetIds 需要删除的数据主键集合
     * @return 结果
     */
    public int deleteBizAssetByAssetIds(Long[] assetIds);
}
