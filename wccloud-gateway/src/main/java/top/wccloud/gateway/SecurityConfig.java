package top.wccloud.gateway;

import cn.hutool.core.util.ObjectUtil;
import cn.hutool.json.JSONUtil;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.amqp.core.AmqpTemplate;
import org.springframework.amqp.rabbit.connection.CorrelationData;
import org.springframework.amqp.rabbit.core.RabbitTemplate;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.web.servlet.FilterRegistrationBean;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.http.HttpHeaders;
import org.springframework.http.server.RequestPath;
import org.springframework.security.authorization.AuthorizationDecision;
import org.springframework.security.authorization.ReactiveAuthorizationManager;
import org.springframework.security.config.Customizer;
import org.springframework.security.config.annotation.web.reactive.EnableWebFluxSecurity;
import org.springframework.security.config.web.server.ServerHttpSecurity;
import org.springframework.security.core.Authentication;
import org.springframework.security.web.server.SecurityWebFilterChain;
import org.springframework.security.web.server.authorization.AuthorizationContext;
import org.springframework.web.cors.CorsConfiguration;
import org.springframework.web.cors.UrlBasedCorsConfigurationSource;
import org.springframework.web.cors.reactive.CorsConfigurationSource;
import org.springframework.web.filter.CorsFilter;
import org.springframework.web.server.ServerWebExchange;
import reactor.core.publisher.Mono;

import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.function.Consumer;

/**
 * 网关统一认证以及授权
 * @author wcz
 */
@Configuration
@EnableWebFluxSecurity
@Slf4j
public class SecurityConfig {

/*    @Resource
    AuthFilter authFilter;*/

    @Resource
    private RabbitTemplate rabbitTemplate;

    @Value("${mq.visit.exchange}")
    private String exchange;
    @Value("${mq.visit.key}")
    private String routingKey;

    @Bean
    public SecurityWebFilterChain filterChain(ServerHttpSecurity security, RedisTemplate<String,Object> redisTemplate) throws Exception {
        security.csrf(ServerHttpSecurity.CsrfSpec::disable);
        security.cors(corsSpec -> corsSpec.configurationSource(exchange -> {
            CorsConfiguration corsConfiguration = new CorsConfiguration();
            corsConfiguration.setAllowCredentials(false); // 是否返回时生成凭证
            corsConfiguration.setAllowedHeaders(List.of("*")); // 允许请求携带哪些请求头信息
            corsConfiguration.setAllowedMethods(List.of("*")); // 允许哪些类型的请求方法
            corsConfiguration.setAllowedOrigins(List.of("*")); // 允许哪些域可以进行方法
            corsConfiguration.setMaxAge(3600L); // 设置预检的最大的时长
            corsConfiguration.setExposedHeaders(Collections.emptyList()); // 设置返回暴露的响应头信息

            return corsConfiguration;
        }));
        security.authorizeExchange(exchangeSpec -> {
            exchangeSpec.pathMatchers("/*/swagger-ui/**").permitAll();
            exchangeSpec.pathMatchers("/*/v3/api-docs/**").permitAll();
            exchangeSpec.pathMatchers("/*/doc.html/**").permitAll();
            exchangeSpec.pathMatchers("/*/webjars/**").permitAll();
            exchangeSpec.pathMatchers("/*/test/a").permitAll();
            exchangeSpec.pathMatchers("/actuator/health").permitAll();
            exchangeSpec.pathMatchers("/*/actuator/health").permitAll();
//            exchangeSpec.pathMatchers("/**").permitAll();
//            exchangeSpec.pathMatchers("/*/wccloud-auth/test/a").permitAll();
            exchangeSpec.pathMatchers("/**").access(new ReactiveAuthorizationManager<AuthorizationContext>() {
                @Override
                public Mono<AuthorizationDecision> check(Mono<Authentication> authentication, AuthorizationContext object) {
                    String path = object.getExchange().getRequest().getPath().toString();
                    String token = null;
                    List<String> tokenList = object.getExchange().getRequest().getHeaders().get("Token");
                    if (tokenList != null) {
                        token = tokenList.getFirst();
                    }
                    Long userId = (Long) redisTemplate.opsForValue().get("accessToken:" + token);

                    HttpHeaders headers = object.getExchange().getRequest().getHeaders();

                    HashMap<String, Object> hashMap = new HashMap<>() {{
                        put("userId", ObjectUtil.defaultIfNull(userId, 0));
                        put("userAgent", ObjectUtil.defaultIfNull(headers.get("User-Agent").getFirst(),""));
                        put("referer", ObjectUtil.defaultIfNull(headers.get("Referer").getFirst(),""));
                        put("host", ObjectUtil.defaultIfNull(headers.get("Host").getFirst(),""));
                        put("ip", ObjectUtil.defaultIfNull(object.getExchange().getRequest().getRemoteAddress().getAddress().getHostAddress(),""));
                        put("path", path);
                    }};
                    String userInfo = JSONUtil.toJsonStr(hashMap);
                    log.info("访问信息：" + userInfo);
                    rabbitTemplate.convertAndSend(exchange,routingKey,userInfo,new CorrelationData());
                    if (path.startsWith("/wccloud-web-rust/anonymous") || path.contains("/auth/login")) {
                        return authentication.thenReturn(new AuthorizationDecision(true));
                    }
                    if (userId != null) {
                        return authentication.thenReturn(new AuthorizationDecision(true));
                    } else {
                        throw new ServiceException(JSONUtil.toJsonStr(Result.error(-2, "未登录，请登录", null)));
                    }
                }
            });
            exchangeSpec.anyExchange().authenticated();
        });
        return security.build();
    }

}
