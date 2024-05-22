package top.wccloud.admin.doman;

import top.wccloud.admin.infrastructure.dao.entity.SysMenuDO;
import lombok.Data;
import lombok.EqualsAndHashCode;

import java.util.List;

/**
 * @author wcz
 */
@Data
@EqualsAndHashCode(callSuper = true)
public class SysMenu extends SysMenuDO {

    private List<SysMenu> children;

}
