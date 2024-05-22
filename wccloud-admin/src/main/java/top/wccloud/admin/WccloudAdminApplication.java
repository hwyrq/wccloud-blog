package top.wccloud.admin;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.openfeign.EnableFeignClients;

@SpringBootApplication(scanBasePackages = "top.wccloud")
@EnableFeignClients
public class WccloudAdminApplication {

    public static void main(String[] args) {
        SpringApplication.run(WccloudAdminApplication.class, args);
    }

}
