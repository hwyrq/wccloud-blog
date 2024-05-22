package top.wccloud.admin.infrastructure.dao.converter;

import top.wccloud.admin.controller.vo.SysMenuVO;
import top.wccloud.admin.doman.SysMenu;
import top.wccloud.admin.infrastructure.dao.entity.SysMenuDO;
import org.mapstruct.Mapper;
import org.mapstruct.factory.Mappers;

import java.util.List;
/**
 * @author wcz
 */
@Mapper
public interface SysMenuConvert {
    SysMenuConvert INSTANCE = Mappers.getMapper(SysMenuConvert.class);

    List<SysMenu> convert(List<SysMenuDO> list);

    List<SysMenuVO> convert1(List<SysMenu> menuList);

    SysMenuDO convert2(SysMenuVO sysMenuVO);
}
