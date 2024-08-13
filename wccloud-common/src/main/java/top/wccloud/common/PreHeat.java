package top.wccloud.common;

import jakarta.annotation.Resource;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.context.event.ApplicationReadyEvent;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.context.event.EventListener;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Component;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.client.RestTemplate;

/**
 * @author wcz
 */
@Component
@Slf4j
@RestController
@RequestMapping("/")
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
        restTemplate.getForEntity("http://127.0.0.1:" + port + "/pre-heat", String.class);
        log.info("预热完毕...");
    }

    @RequestMapping("/pre-heat")
    public void process() {
    }

}
