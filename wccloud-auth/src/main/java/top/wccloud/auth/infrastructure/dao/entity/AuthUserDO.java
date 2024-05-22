package top.wccloud.auth.infrastructure.dao.entity;

import com.baomidou.mybatisplus.annotation.*;
import lombok.Data;
import lombok.EqualsAndHashCode;
import org.springframework.validation.annotation.Validated;
import top.wccloud.common.BaseDO;

import java.time.LocalDateTime;
/**
 * @author wcz
 */
@EqualsAndHashCode(callSuper = true)
@Data
@Validated
@TableName(value = "auth_user")
public class AuthUserDO extends BaseDO {

    @TableId
    private Long userId;//

    private String username;//

    private String password;//

    private String nickname;//

    private String remark;//

    private String email;//

    private String mobile;//

    private String sex;//

    private String avatar;//

    private Integer status;//

    private String loginIp;//

    private LocalDateTime loginTime;//


}


