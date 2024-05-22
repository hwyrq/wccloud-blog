package top.wccloud.common.dto;

import io.swagger.v3.oas.annotations.media.Schema;
import jakarta.validation.constraints.NotBlank;
import lombok.Data;
import org.hibernate.validator.constraints.Length;

import java.io.Serializable;
import java.time.LocalDateTime;

@Data
public class AuthUserDTO implements Serializable {
    @Schema(defaultValue = "wang")
    private String username;//

    @Schema(defaultValue = "123")
    private String password;//

    @Schema(defaultValue = "王")
    private String nickname;//

    private String remark;//

    private String email;//

    private String mobile;//
    private LocalDateTime createTime;//
}
