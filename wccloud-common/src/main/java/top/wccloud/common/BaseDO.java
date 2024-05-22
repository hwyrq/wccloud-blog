package top.wccloud.common;

import com.baomidou.mybatisplus.annotation.FieldFill;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableLogic;
import lombok.Data;

import java.io.Serializable;
import java.time.LocalDateTime;

/**
 * baseEntity
 * @author wcz
 */
@Data
public class BaseDO implements Serializable {
    @TableField(fill = FieldFill.INSERT)
    private Long createUserId;//

    @TableField(fill = FieldFill.INSERT_UPDATE)
    private Long updateUserId;//

    @TableField(fill = FieldFill.INSERT)
    private LocalDateTime createTime;//

    @TableField(fill = FieldFill.INSERT_UPDATE)
    private LocalDateTime updateTime;//

    @TableLogic
    private Boolean deleted;//
}
