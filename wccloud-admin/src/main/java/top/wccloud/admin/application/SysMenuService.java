package top.wccloud.admin.application;


import top.wccloud.admin.controller.vo.SysMenuVO;

import java.util.List;
/**
 * @author wcz
 */
public interface SysMenuService {

    List<SysMenuVO> listTree0();

    List<SysMenuVO> listTree();

    Boolean save(SysMenuVO sysMenuVO);

    Boolean update(SysMenuVO sysMenuVO);

    Boolean delete(Long menuId);
}
