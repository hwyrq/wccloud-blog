package top.wccloud.auth.application;

import jakarta.servlet.http.HttpServletRequest;
import top.wccloud.auth.controller.vo.AuthUserVO;
import top.wccloud.auth.controller.vo.LoginRespVO;
import top.wccloud.common.Result;
import top.wccloud.common.dto.ResetPwdDTO;

/**
 * @author wcz
 */
public interface AuthUserService {
    void register(AuthUserVO user);

    AuthUserVO testCache(AuthUserVO userVO);

    Result<LoginRespVO> login(AuthUserVO user);


    Result<LoginRespVO> logout(String token);

    Result<Boolean> resetPwd(ResetPwdDTO resetPwdDTO);
}
