package top.wccloud.auth.doman;

import com.baomidou.mybatisplus.extension.service.IService;
import org.springframework.cache.annotation.Cacheable;
import top.wccloud.auth.infrastructure.dao.entity.AuthUserDO;

import java.io.Serializable;

/**
 * @author wcz
 */
public interface AuthUserRepository extends IService<AuthUserDO> {
    /**
     * 获取用户信息
     * @param id 主键ID
     * @return
     */
    @Override
    AuthUserDO getById(Serializable id);

    /**
     * 获取用户信息
     * @param username 用户名
     * @return
     */
    AuthUserDO selectByUsername(String username);

    /**
     * 校验用户存在，校验密码
     * @param user
     */
    void loginCheck(AuthUser user);

    /**
     * 生成token 并保存
     * @param user
     */
    void loginToken(AuthUser user);

    /**
     * 登出
     * @param accessToken
     */
    void logoutToken(String accessToken);

    /**
     * 登录后并赋值
     * @param sysUser
     */
    void loginData(AuthUser sysUser);

    /**
     * 更新用户
     * @param entity 实体对象
     * @return
     */
    @Override
    boolean updateById(AuthUserDO entity);
}
