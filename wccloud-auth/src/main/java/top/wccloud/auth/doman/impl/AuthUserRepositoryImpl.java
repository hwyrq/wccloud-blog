package top.wccloud.auth.doman.impl;

import cn.hutool.core.lang.UUID;
import cn.hutool.crypto.digest.DigestUtil;
import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.extension.service.IService;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.cache.annotation.CacheConfig;
import org.springframework.cache.annotation.CacheEvict;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Service;
import top.wccloud.auth.controller.vo.LoginRespVO;
import top.wccloud.auth.doman.AuthUser;
import top.wccloud.auth.doman.AuthUserRepository;
import top.wccloud.auth.infrastructure.dao.entity.AuthUserDO;
import top.wccloud.auth.infrastructure.dao.mapper.AuthUserMapper;
import top.wccloud.common.ServiceException;

import java.time.LocalDateTime;
import java.util.List;
import java.util.concurrent.TimeUnit;
/**
 * @author wcz
 */
@Service
@Slf4j
@CacheConfig(cacheNames = "auth_user", keyGenerator = "cacheKeyGenerator")
public class AuthUserRepositoryImpl extends ServiceImpl<AuthUserMapper, AuthUserDO> implements AuthUserRepository, IService<AuthUserDO> {

    @Resource
    private RedisTemplate<String, Object> redisTemplate;

    @Resource
    private AuthUserRepository authUserRepository;

    @Cacheable(sync = true,key = "#root.methodName+':'+#username")
    @Override
    public AuthUserDO selectByUsername(String username) {

        List<AuthUserDO> list = list(new LambdaQueryWrapper<AuthUserDO>().eq(AuthUserDO::getUsername,username));
        if (list.isEmpty()) {
            return null;
        }

        return list.getFirst();
    }

    @Override
    public void loginCheck(AuthUser user) {
        AuthUserDO authUserDO = authUserRepository.selectByUsername(user.getUsername());
        if (authUserDO == null) {
            throw new ServiceException("用户不存在！");
        }
        boolean bcryptCheck = DigestUtil.bcryptCheck(user.getPassword(), authUserDO.getPassword());
        if (!bcryptCheck) {
            throw new ServiceException("密码不正确！");
        }
    }

    @Override
    public void loginToken(AuthUser user) {
        String accessToken = UUID.fastUUID().toString().replace("-","");
        String refreshToken = UUID.fastUUID().toString().replace("-","");
        user.setAccessToken(accessToken);
        user.setRefreshToken(refreshToken);
        user.setExpireTime(LocalDateTime.now().plusMinutes(30));
        redisTemplate.opsForSet().add("accessToken", accessToken, user.getUserId());
        redisTemplate.opsForSet().add("refreshToken", refreshToken, user.getUserId());
        redisTemplate.opsForValue().set("accessToken:" + accessToken, user.getUserId(), 30, TimeUnit.HOURS);
        redisTemplate.opsForValue().set("refreshToken:" + refreshToken, 1, 300, TimeUnit.HOURS);
    }

    @Override
    public void logoutToken(String accessToken) {
        redisTemplate.delete("accessToken:" + accessToken);
        LoginRespVO loginRespVO = (LoginRespVO) redisTemplate.opsForValue().get("accessToken:" + accessToken);
        if (loginRespVO != null) {
            redisTemplate.delete("refreshToken:" + loginRespVO.getRefreshToken());
        }
    }

    @Override
    public void loginData(AuthUser sysUser) {
        AuthUserDO authUserDO = authUserRepository.selectByUsername(sysUser.getUsername());
        sysUser.setUserId(authUserDO.getUserId());
        sysUser.setNickname(authUserDO.getNickname());
    }

    @CacheEvict(allEntries = true)
    @Override
    public boolean save(AuthUserDO entity) {
        AuthUserDO userDO = authUserRepository.selectByUsername(entity.getUsername());
        if (userDO != null) {
            throw new ServiceException("用户名已存在");
        }
        return super.save(entity);
    }
}
