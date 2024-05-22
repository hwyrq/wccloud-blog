package top.wccloud.auth.infrastructure.dao.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import org.apache.ibatis.annotations.Mapper;
import top.wccloud.auth.infrastructure.dao.entity.AuthUserDO;
/**
 * @author wcz
 */
@Mapper
public interface AuthUserMapper extends BaseMapper<AuthUserDO> {

}
