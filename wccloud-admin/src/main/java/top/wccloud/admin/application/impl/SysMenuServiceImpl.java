package top.wccloud.admin.application.impl;

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
public class SysMenuServiceImpl implements SysMenuService {
    @Resource
    private SysMenuRepository sysMenuRepository;

    @Resource
    private SysMenuMapper sysMenuMapper;


    @Override
    public List<SysMenuVO> listTree0() {
        return SysMenuConvert.INSTANCE.convert1(sysMenuRepository.listTree0());
    }

    @Override
    public List<SysMenuVO> listTree() {
        return SysMenuConvert.INSTANCE.convert1(sysMenuRepository.listTree());
    }

    @Override
    public Boolean save(SysMenuVO sysMenuVO) {
        return sysMenuRepository.save(SysMenuConvert.INSTANCE.convert2(sysMenuVO));
    }

    @Override
    public Boolean update(SysMenuVO sysMenuVO) {
        return sysMenuRepository.updateById(SysMenuConvert.INSTANCE.convert2(sysMenuVO));
    }

    @Override
    public Boolean delete(Long menuId) {
        return sysMenuRepository.removeById(menuId);
    }


}
