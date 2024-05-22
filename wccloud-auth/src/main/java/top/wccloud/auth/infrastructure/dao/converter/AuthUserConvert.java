package top.wccloud.auth.infrastructure.dao.converter;

import org.mapstruct.Mapper;
import org.mapstruct.factory.Mappers;
import top.wccloud.auth.controller.vo.AuthUserVO;
import top.wccloud.auth.controller.vo.LoginRespVO;
import top.wccloud.auth.doman.AuthUser;
import top.wccloud.auth.infrastructure.dao.entity.AuthUserDO;
/**
 * @author wcz
 */
@Mapper
public interface AuthUserConvert {
    AuthUserConvert INSTANCE = Mappers.getMapper(AuthUserConvert.class);

    AuthUser convert(AuthUserDO authUserDO);

    AuthUserDO convert(AuthUserVO authUserVO);

    AuthUser convert1(AuthUserVO user);

    LoginRespVO convert2(AuthUser sysUser);
}
