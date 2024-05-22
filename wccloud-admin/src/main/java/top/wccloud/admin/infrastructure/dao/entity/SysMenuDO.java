package top.wccloud.admin.infrastructure.dao.entity;

import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;
import lombok.EqualsAndHashCode;
import top.wccloud.common.BaseDO;
/**
 * @author wcz
 */
@EqualsAndHashCode(callSuper = true)
@Data
@TableName("sys_menu")
public class SysMenuDO extends BaseDO {
    @TableId
    private Long menuId;//
    private String menuName;//
    private String permission;//
    private Integer type;//
    private Integer sort;//
    private Long parentId;//
    private String path;//
    private String icon;//
    private Integer status;//
    private Boolean visible;//

}
