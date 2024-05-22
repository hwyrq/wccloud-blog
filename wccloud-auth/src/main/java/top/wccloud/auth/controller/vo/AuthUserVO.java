package top.wccloud.auth.controller.vo;


import io.swagger.v3.oas.annotations.media.Schema;
import jakarta.validation.constraints.NotBlank;
import lombok.Data;
import org.hibernate.validator.constraints.Length;

import java.io.Serializable;
import java.time.LocalDateTime;
/**
 * @author wcz
 */
@Data
public class AuthUserVO implements Serializable {

    @NotBlank
    @Length(min = 5,message = "用户名至少5位")
    @Length(max = 32,message = "用户名最多32位")
    @Schema(defaultValue = "wang")
    private String username;//

    @NotBlank
    @Length(min = 5,message = "密码至少5位")
    @Length(max = 32,message = "密码最多32位")
    @Schema(defaultValue = "123")
    private String password;//

    @Schema(defaultValue = "王")
    private String nickname;//

    private String remark;//

    private String email;//

    private String mobile;//
    private LocalDateTime createTime;//

}
