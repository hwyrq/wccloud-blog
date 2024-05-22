package top.wccloud.common;

import cn.hutool.core.util.StrUtil;
import cn.hutool.json.JSONUtil;
import org.springframework.boot.autoconfigure.cache.CacheProperties;
import org.springframework.cache.annotation.EnableCaching;
import org.springframework.cache.interceptor.KeyGenerator;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.Primary;
import org.springframework.data.redis.cache.RedisCacheConfiguration;
import org.springframework.data.redis.serializer.RedisSerializationContext;

import static top.wccloud.common.RedisConfig.buildRedisSerializer;

/**
 * spring cache 全局配置
 * @author wcz
 */
@EnableCaching
@Configuration
public class SpringCacheConfig {


    @Bean
    @Primary
    public RedisCacheConfiguration redisCacheConfiguration(CacheProperties cacheProperties) {
        CacheProperties.Redis redis = cacheProperties.getRedis();
        RedisCacheConfiguration config = RedisCacheConfiguration.defaultCacheConfig();
        config = config.computePrefixWith(cacheName -> cacheName + StrUtil.COLON);

        config = config.serializeValuesWith(RedisSerializationContext.SerializationPair.fromSerializer(buildRedisSerializer()));
        config = config.entryTtl(redis.getTimeToLive());
        config = config.prefixCacheNameWith(redis.getKeyPrefix());
        if (!redis.isCacheNullValues()) {
            config = config.disableCachingNullValues();
        }
        if (!redis.isUseKeyPrefix()) {
            config = config.disableKeyPrefix();
        }
        return config;
    }


    @Bean
    public KeyGenerator cacheKeyGenerator() {
        return (target, method, params) -> method.getName() + JSONUtil.toJsonStr(params);
    }
}
