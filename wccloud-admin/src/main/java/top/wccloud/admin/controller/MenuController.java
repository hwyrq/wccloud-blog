package top.wccloud.admin.controller;

import top.wccloud.admin.application.SysMenuService;
import top.wccloud.admin.controller.vo.SysMenuVO;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.annotation.Resource;
import jakarta.validation.constraints.NotNull;
import jakarta.validation.groups.Default;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;
import top.wccloud.common.Result;
import top.wccloud.common.SaveValid;
import top.wccloud.common.UpdateValid;

import java.util.List;
/**
 * @author wcz
 */
@RestController
@RequestMapping("/menu")
@Tag(name = "菜单")
@Validated
public class MenuController {

    @Resource
    private SysMenuService sysMenuService;

    @GetMapping("/list-status0")
    @Operation(summary = "查询所有开启菜单")
    public Result<List<SysMenuVO>> listStatus0() {
        return Result.success("", sysMenuService.listTree0());
    }
    @GetMapping("/list")
    @Operation(summary = "查询所有菜单")
    public Result<List<SysMenuVO>> list() {
        return Result.success("", sysMenuService.listTree());
    }
    @PostMapping("/save")
    @Operation(summary = "保存菜单")
    public Result<Boolean> save( @Validated(value = {SaveValid.class, Default.class}) @RequestBody SysMenuVO sysMenuVO) {
        return Result.success("", sysMenuService.save(sysMenuVO));
    }

    @PutMapping("/update")
    @Operation(summary = "更新菜单")
    public Result<Boolean> update(@Validated(value = {UpdateValid.class,Default.class}) @RequestBody SysMenuVO sysMenuVO) {
        return Result.success("", sysMenuService.update(sysMenuVO));
    }

    @DeleteMapping("/delete")
    @Operation(summary = "删除菜单")
    public Result<Boolean> delete(@NotNull Long menuId) {
        return Result.success("", sysMenuService.delete(menuId));
    }
}
