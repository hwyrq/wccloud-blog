package top.wccloud.gateway;

import cn.hutool.json.JSONUtil;
import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.web.servlet.FilterRegistrationBean;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.data.redis.core.RedisTemplate;
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
import java.util.List;
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
            exchangeSpec.pathMatchers("/*/auth/**").permitAll();
            exchangeSpec.pathMatchers("/*/test/a").permitAll();
//            exchangeSpec.pathMatchers("/**").permitAll();
//            exchangeSpec.pathMatchers("/*/wccloud-auth/test/a").permitAll();
            exchangeSpec.pathMatchers("/**").access(new ReactiveAuthorizationManager<AuthorizationContext>() {
                @Override
                public Mono<AuthorizationDecision> check(Mono<Authentication> authentication, AuthorizationContext object) {
                    RequestPath path = object.getExchange().getRequest().getPath();
                    String value = object.getExchange().getRequest().getPath().toString();
                    log.info(value);
                    if (value.startsWith("/wccloud-web-rust/anonymous")) {
                        return authentication.thenReturn(new AuthorizationDecision(true));
                    }
                    List<String> tokenList = object.getExchange().getRequest().getHeaders().get("Token");
                    if (tokenList != null) {
                        String tokenValue = tokenList.getFirst();
                        Boolean hasKey = redisTemplate.hasKey("accessToken:" + tokenValue);
                        if (hasKey == null || !hasKey) {
                            throw new ServiceException(JSONUtil.toJsonStr(Result.error(-2, "未登录，请登录", null)));
                        }

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
