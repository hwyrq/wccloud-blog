package top.wccloud.admin.controller.vo;


import lombok.Data;

import java.time.LocalDateTime;
/**
 * @author wcz
 */
@Data
public class LoginRespVO {

    private Long userId;

    private String nickname;

    private String accessToken;
    private String refreshToken;
    private LocalDateTime expireTime;

}
