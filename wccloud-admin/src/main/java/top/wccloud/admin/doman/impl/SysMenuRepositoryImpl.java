package top.wccloud.admin.doman.impl;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.extension.service.IService;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import top.wccloud.admin.doman.SysMenu;
import top.wccloud.admin.doman.SysMenuRepository;
import top.wccloud.admin.infrastructure.dao.converter.SysMenuConvert;
import top.wccloud.admin.infrastructure.dao.entity.SysMenuDO;
import top.wccloud.admin.infrastructure.dao.mapper.SysMenuMapper;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Objects;
/**
 * @author wcz
 */
@Service
public class SysMenuRepositoryImpl extends ServiceImpl<SysMenuMapper, SysMenuDO> implements SysMenuRepository, IService<SysMenuDO> {


    @Override
    public List<SysMenuDO> listStatus(int status) {
        return list(new LambdaQueryWrapper<SysMenuDO>().eq(SysMenuDO::getStatus, status));
    }

    @Override
    public List<SysMenu> listTree() {
        List<SysMenu> menuList = SysMenuConvert.INSTANCE.convert( list());
        SysMenu sysMenu = new SysMenu();
        sysMenu.setMenuId(0L);
        menus(sysMenu,menuList);
        return sysMenu.getChildren();
    }

    @Override
    public List<SysMenu> listTree0() {
        List<SysMenu> menuList = SysMenuConvert.INSTANCE.convert( listStatus(0));
        SysMenu sysMenu = new SysMenu();
        sysMenu.setMenuId(0L);
        menus(sysMenu,menuList);
        return sysMenu.getChildren();
    }

    private void menus(SysMenu menu, List<SysMenu> list) {
        List<SysMenu> menuList = list.stream()
                .filter(sysMenuDO -> Objects.equals(sysMenuDO.getParentId(), menu.getMenuId())).toList();
        if (!menuList.isEmpty()) {
            menu.setChildren(menuList);
        }
        for (SysMenu sysMenu : menuList) {
            menus(sysMenu, list);
        }
    }
}
