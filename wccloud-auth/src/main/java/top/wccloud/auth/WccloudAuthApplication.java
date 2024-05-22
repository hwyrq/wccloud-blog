package top.wccloud.auth;

import org.mybatis.spring.annotation.MapperScan;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cache.annotation.EnableCaching;

@SpringBootApplication(scanBasePackages = "top.wccloud")
public class WccloudAuthApplication {

    public static void main(String[] args) {
        SpringApplication.run(WccloudAuthApplication.class, args);
    }

}
