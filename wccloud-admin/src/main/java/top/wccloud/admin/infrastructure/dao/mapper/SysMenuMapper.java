package top.wccloud.admin.infrastructure.dao.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import top.wccloud.admin.infrastructure.dao.entity.SysMenuDO;
import org.apache.ibatis.annotations.Mapper;
/**
 * @author wcz
 */
@Mapper
public interface SysMenuMapper extends BaseMapper<SysMenuDO> {

    void deleteAllByAuto();

    void unDelete(SysMenuDO menuDO);

    SysMenuDO selectByPathCreate(SysMenuDO sysMenuDO);
}
