package top.wccloud.admin.controller.vo;

import top.wccloud.admin.doman.SysMenu;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotNull;
import jakarta.validation.constraints.Null;
import lombok.Data;
import top.wccloud.common.SaveValid;
import top.wccloud.common.UpdateValid;

import java.util.List;
/**
 * @author wcz
 */
@Data
//@EqualsAndHashCode(callSuper = true)

public class SysMenuVO {
    private List<SysMenu> children;

    @NotNull(groups = {UpdateValid.class})
    @Null(groups = {SaveValid.class})
    private Long menuId;//
    @NotBlank(message = "名称不能为空")
    private String menuName;//
    private String permission;//
    @NotNull(message = "类型不能为空")
    private Integer type;//
    private Integer sort;//

    private Long parentId;//
    private String path;//
    private String icon;//
    private Integer status;//
    private Boolean visible;//

}
