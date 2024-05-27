package top.wccloud.admin.controller;

import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;
import org.springframework.stereotype.Controller;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.ResponseBody;
import top.wccloud.admin.infrastructure.fegin.AuthFeignClient;
import top.wccloud.common.dto.AuthUserDTO;
import top.wccloud.common.dto.LoginRespDTO;
import top.wccloud.common.Result;
import top.wccloud.common.dto.ResetPwdDTO;

/**
 * @author wcz
 */
@Controller
@RequestMapping("/auth")
@Validated
@Tag(name = "认证")
public class AuthController {

    @Resource
    AuthFeignClient authUserService;

    @Operation(summary = "注册")
    @PostMapping("/register")
    @ResponseBody
    public Result<?> register(@Validated @RequestBody AuthUserDTO user) {
        authUserService.register(user);
        return Result.success("注册成功！");
    }

    @Operation(summary = "登录")
    @PostMapping("/login")
    @ResponseBody
    public Result<LoginRespDTO> login(@Validated @RequestBody AuthUserDTO user) {
        return authUserService.login(user);
    }

    @Operation(summary = "登出")
    @PostMapping("/logout")
    @ResponseBody
    public Result<LoginRespDTO> logout(HttpServletRequest request) {
        return authUserService.logout(request.getHeader("Token"));
    }

    @Operation(summary = "重置密码")
    @PostMapping("/reset-pwd")
    @ResponseBody
    public Result<Boolean> resetPwd(@Validated @RequestBody ResetPwdDTO resetPwdDTO,HttpServletRequest request) {
        String token = request.getHeader("Token");
        resetPwdDTO.setToken(token);
        return authUserService.resetPwd(resetPwdDTO);
    }
}
