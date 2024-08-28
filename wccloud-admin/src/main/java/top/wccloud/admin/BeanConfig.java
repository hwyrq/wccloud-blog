package top.wccloud.admin;


import lombok.SneakyThrows;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.io.ClassPathResource;


@Configuration
public class BeanConfig {

    @SneakyThrows
    @Bean
    public byte[] ipBytes() {
        return new ClassPathResource("ip2region.xdb").getInputStream().readAllBytes();
    }

}
