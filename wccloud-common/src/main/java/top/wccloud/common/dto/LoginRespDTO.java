package top.wccloud.common.dto;


import lombok.Data;

import java.time.LocalDateTime;

@Data
public class LoginRespDTO {

    private Long userId;

    private String nickname;

    private String accessToken;
    private String refreshToken;
    private LocalDateTime expireTime;

}
