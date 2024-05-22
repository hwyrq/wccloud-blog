package top.wccloud.auth.doman;

import com.baomidou.mybatisplus.extension.service.IService;
import top.wccloud.auth.infrastructure.dao.entity.AuthUserDO;

/**
 * @author wcz
 */
public interface AuthUserRepository extends IService<AuthUserDO> {
    AuthUserDO selectByUsername(String username);

    void loginCheck(AuthUser user);

    void loginToken(AuthUser user);

    void logoutToken(String accessToken);

    void loginData(AuthUser sysUser);
}
