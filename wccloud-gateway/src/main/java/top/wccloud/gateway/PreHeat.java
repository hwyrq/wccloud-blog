package top.wccloud.gateway;

import jakarta.annotation.Resource;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.context.event.ApplicationReadyEvent;
import org.springframework.context.event.EventListener;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Component;
import org.springframework.web.client.RestTemplate;

/**
 * @author wcz
 */
@Component
@Slf4j
public class PreHeat {

    @Value("${server.port}")
    private String port;

    @Resource
    private RedisTemplate<String,Object> redisTemplate;

    @SneakyThrows
    @EventListener(ApplicationReadyEvent.class)
    public void init() {
        redisTemplate.opsForValue().get("test");

        RestTemplate restTemplate = new RestTemplate();
        try {
            restTemplate.getForEntity("http://127.0.0.1:" + port + "/wccloud-admin-server/actuator/health", String.class);
        } catch (Exception ignored) {
            log.error("webflux预热失败，未发现wccloud-admin-server");

        }
        log.info("预热完毕...");
    }

}
