package top.wccloud.common.dto;


import jakarta.validation.constraints.NotBlank;
import lombok.Data;
import org.hibernate.validator.constraints.Length;

import java.time.LocalDateTime;

@Data
public class ResetPwdDTO {

    @NotBlank(message = "旧密码不能为空")
    private String pwd;
    @NotBlank
    @Length(min = 5,message = "密码至少5位")
    @Length(max = 32,message = "密码最多32位")
    private String newPwd;

    private String token;

}
