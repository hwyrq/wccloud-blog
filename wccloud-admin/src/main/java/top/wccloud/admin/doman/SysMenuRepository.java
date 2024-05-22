package top.wccloud.admin.doman;

import com.baomidou.mybatisplus.extension.service.IService;
import top.wccloud.admin.infrastructure.dao.entity.SysMenuDO;

import java.util.List;
/**
 * @author wcz
 */
public interface SysMenuRepository extends IService<SysMenuDO> {

    List<SysMenuDO> listStatus(int status);

    List<SysMenu> listTree();

    List<SysMenu> listTree0();
}
