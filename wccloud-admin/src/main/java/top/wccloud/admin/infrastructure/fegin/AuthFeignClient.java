package top.wccloud.admin.infrastructure.fegin;

import org.springframework.cloud.openfeign.FeignClient;
import org.springframework.web.bind.annotation.PostMapping;
import top.wccloud.common.dto.AuthUserDTO;
import top.wccloud.common.dto.LoginRespDTO;
import top.wccloud.common.Result;
/**
 * @author wcz
 */
@FeignClient(name = "wccloud-auth-server")
public interface AuthFeignClient {

    @PostMapping("/auth/register")
     Result<?> register(AuthUserDTO user);

    @PostMapping("/auth/login")
     Result<LoginRespDTO> login(AuthUserDTO user);

    @PostMapping("/auth/logout")
     Result<LoginRespDTO> logout(String token);

}
