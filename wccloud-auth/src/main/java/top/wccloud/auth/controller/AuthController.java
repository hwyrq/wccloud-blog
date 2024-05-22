package top.wccloud.auth.controller;

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
import top.wccloud.auth.application.AuthUserService;
import top.wccloud.auth.controller.vo.LoginRespVO;
import top.wccloud.auth.controller.vo.AuthUserVO;
import top.wccloud.common.Result;

/**
 * @author wcz
 */
@Controller
@RequestMapping("/auth")
@Validated
@Tag(name = "认证")
public class AuthController {

    @Resource
    AuthUserService authUserService;

    @Operation(summary = "注册")
    @PostMapping("/register")
    @ResponseBody
    public Result<?> register(@Validated @RequestBody AuthUserVO user) {
        authUserService.register(user);
        return Result.success("注册成功！");
    }

    @Operation(summary = "登录")
    @PostMapping("/login")
    @ResponseBody
    public Result<LoginRespVO> login(@Validated @RequestBody AuthUserVO user) {
        return authUserService.login(user);
    }

    @Operation(summary = "登出")
    @PostMapping("/logout")
    @ResponseBody
    public Result<LoginRespVO> logout(String token) {
        return authUserService.logout(token);
    }
}
