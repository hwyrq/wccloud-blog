package top.wccloud.auth.doman;

import lombok.Data;
import lombok.EqualsAndHashCode;
import top.wccloud.auth.infrastructure.dao.entity.AuthUserDO;

import java.time.LocalDateTime;
/**
 * @author wcz
 */
@Data
@EqualsAndHashCode(callSuper = true)
public class AuthUser extends AuthUserDO {
    private String accessToken;
    private String refreshToken;
    private LocalDateTime expireTime;
}
