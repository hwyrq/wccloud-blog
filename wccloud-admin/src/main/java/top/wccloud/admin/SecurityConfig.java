package top.wccloud.admin;

import lombok.SneakyThrows;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configurers.AbstractHttpConfigurer;
import org.springframework.security.web.SecurityFilterChain;
/**
 * @author wcz
 */
@Configuration
public class SecurityConfig {

    @SneakyThrows
    @Bean
    public SecurityFilterChain security(HttpSecurity httpSecurity) {
        httpSecurity.csrf(AbstractHttpConfigurer::disable);
        httpSecurity.cors(httpSecurityCorsConfigurer -> httpSecurityCorsConfigurer.disable());
        httpSecurity.authorizeHttpRequests(matcherRegistry -> {
            matcherRegistry.requestMatchers("/auth/**").permitAll();
            matcherRegistry.requestMatchers("/webjars/**").permitAll();
            matcherRegistry.requestMatchers("/doc.html/**").permitAll();
            matcherRegistry.requestMatchers("/v3/api-docs/**").permitAll();
            matcherRegistry.requestMatchers("/**").permitAll();
            matcherRegistry.anyRequest().authenticated();
        });

        return httpSecurity.build();
    }
}
