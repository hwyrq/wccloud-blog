package top.wccloud.auth;

import jakarta.servlet.Filter;
import jakarta.servlet.http.HttpServletRequest;
import lombok.SneakyThrows;
import org.springframework.boot.web.servlet.FilterRegistrationBean;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.Customizer;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configurers.AbstractHttpConfigurer;
import org.springframework.security.config.annotation.web.configurers.CorsConfigurer;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.web.cors.CorsConfiguration;
import org.springframework.web.cors.CorsConfigurationSource;
import org.springframework.web.cors.UrlBasedCorsConfigurationSource;
import org.springframework.web.filter.CorsFilter;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;
/**
 * @author wcz
 */
@Configuration
public class SecurityConfig {

    @SneakyThrows
    @Bean
    public SecurityFilterChain security(HttpSecurity httpSecurity) {
        httpSecurity.csrf(AbstractHttpConfigurer::disable);
        httpSecurity.cors(AbstractHttpConfigurer::disable);
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
