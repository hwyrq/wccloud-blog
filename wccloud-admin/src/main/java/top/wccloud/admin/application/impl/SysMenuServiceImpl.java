package top.wccloud.admin.application.impl;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.cache.annotation.CacheConfig;
import org.springframework.cache.annotation.CacheEvict;
import org.springframework.cache.annotation.Cacheable;
import top.wccloud.admin.application.SysMenuService;
import top.wccloud.admin.controller.vo.SysMenuVO;
import top.wccloud.admin.doman.SysMenuRepository;
import top.wccloud.admin.infrastructure.dao.converter.SysMenuConvert;
import top.wccloud.admin.infrastructure.dao.mapper.SysMenuMapper;
import jakarta.annotation.Resource;
import org.springframework.stereotype.Service;

import java.util.List;

/**
 * @author wcz
 */
@Service
@CacheConfig(cacheNames = "SysMenuServiceImpl",keyGenerator = "cacheKeyGenerator")
public class SysMenuServiceImpl implements SysMenuService {
    private static final Logger log = LoggerFactory.getLogger(SysMenuServiceImpl.class);
    @Resource
    private SysMenuRepository sysMenuRepository;

    @Resource
    private SysMenuMapper sysMenuMapper;


    @Override
    @Cacheable(sync = true)
    public List<SysMenuVO> listTree0() {
        return SysMenuConvert.INSTANCE.convert1(sysMenuRepository.listTree0());
    }

    @Override
    @Cacheable(sync = true)
    public List<SysMenuVO> listTree() {
        return SysMenuConvert.INSTANCE.convert1(sysMenuRepository.listTree());
    }

    @Override
    @CacheEvict(allEntries = true)
    public Boolean save(SysMenuVO sysMenuVO) {
        return sysMenuRepository.save(SysMenuConvert.INSTANCE.convert2(sysMenuVO));
    }

    @Override
    @CacheEvict(allEntries = true)
    public Boolean update(SysMenuVO sysMenuVO) {
        return sysMenuRepository.updateById(SysMenuConvert.INSTANCE.convert2(sysMenuVO));
    }

    @Override
    @CacheEvict(allEntries = true)
    public Boolean delete(Long menuId) {
        return sysMenuRepository.removeById(menuId);
    }


}
