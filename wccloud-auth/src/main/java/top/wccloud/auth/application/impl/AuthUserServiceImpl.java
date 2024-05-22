package top.wccloud.auth.application.impl;

import cn.hutool.crypto.digest.DigestUtil;
import cn.hutool.json.JSONUtil;
import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpSession;
import lombok.extern.slf4j.Slf4j;
import org.springframework.cache.annotation.CacheConfig;
import org.springframework.cache.annotation.CacheEvict;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Service;
import top.wccloud.auth.application.AuthUserService;
import top.wccloud.auth.controller.vo.AuthUserVO;
import top.wccloud.auth.controller.vo.LoginRespVO;
import top.wccloud.auth.doman.AuthUser;
import top.wccloud.auth.doman.AuthUserRepository;
import top.wccloud.auth.infrastructure.dao.converter.AuthUserConvert;
import top.wccloud.auth.infrastructure.dao.mapper.AuthUserMapper;
import top.wccloud.common.Result;
/**
 * @author wcz
 */
@Service
@Slf4j
@CacheConfig(cacheNames = "auth_user", keyGenerator = "cacheKeyGenerator")
public class AuthUserServiceImpl implements AuthUserService {

    @Resource
    private AuthUserRepository authUserRepository;

    @Resource
    private AuthUserMapper authUserMapper;

    @Resource
    private HttpSession httpSession;

    @Resource
    private RedisTemplate<String,Object> redisTemplate;

    @Override
    @CacheEvict(allEntries = true)
    public void register(AuthUserVO user) {
        user.setPassword(DigestUtil.bcrypt(user.getPassword()));
        authUserRepository.save(AuthUserConvert.INSTANCE.convert(user));
    }

    @Override
    @Cacheable(sync = true)
    public AuthUserVO testCache(AuthUserVO userVO) {
        log.info(JSONUtil.toJsonStr(userVO));
        return userVO;
    }

    @Override
    public Result<LoginRespVO> login(AuthUserVO user) {
        AuthUser sysUser = AuthUserConvert.INSTANCE.convert1(user);
        //校验登录
        authUserRepository.loginCheck(sysUser);
        //生成token
        authUserRepository.loginToken(sysUser);
        //赋值数据
        authUserRepository.loginData(sysUser);

        return Result.success("登录成功！", AuthUserConvert.INSTANCE.convert2(sysUser) );
    }

    @Override
    public Result<LoginRespVO> logout(String token) {
        authUserRepository.logoutToken(token);
        return Result.success("登出成功！", null);
    }
}
