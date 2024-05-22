package top.wccloud.admin;

import cn.hutool.core.annotation.AnnotationUtil;
import cn.hutool.extra.spring.SpringUtil;
import cn.hutool.json.JSONUtil;
import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;
import org.springframework.context.ApplicationContext;
import org.springframework.stereotype.Component;
import org.springframework.stereotype.Controller;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import top.wccloud.admin.infrastructure.dao.entity.SysMenuDO;
import top.wccloud.admin.infrastructure.dao.mapper.SysMenuMapper;

import java.lang.annotation.Annotation;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;
/**
 * 基于controller自动扫描添加菜单
 * @author wcz
 */
@Component
public class ScanMenu {

    @Resource
    SysMenuMapper sysMenuMapper;

    @PostConstruct
    @Transactional(rollbackFor = Exception.class)
    public void run() {
        ApplicationContext context = SpringUtil.getApplicationContext();
        String[] definitionNames = context.getBeanDefinitionNames();
        sysMenuMapper.deleteAllByAuto();

        for (String definitionName : definitionNames) {
            Object bean = context.getBean(definitionName);
            Class<?> aClass = bean.getClass();
            boolean cglib = aClass.getName().contains("CGLIB");
            if (cglib) {
                aClass = aClass.getSuperclass();
            }
            Controller controller = AnnotationUtil.getAnnotation(aClass, Controller.class);
            RestController restController = AnnotationUtil.getAnnotation(aClass, RestController.class);

            if (controller == null && restController == null) {
                continue;
            }
            RequestMapping requestMapping = AnnotationUtil.getAnnotation(aClass, RequestMapping.class);
            if (requestMapping == null) {
                    continue;
                }
                String[] value = requestMapping.value();
                if (value.length != 1) {
                    continue;
                }
                Tag tag = AnnotationUtil.getAnnotation(aClass, Tag.class);
                if (tag == null) {
                    continue;
                }
                String name = tag.name();
                if (name == null) {
                    continue;
                }
                System.out.println("菜单名：" + name + "; 路径：" + value[0]);
            SysMenuDO d = new SysMenuDO();
            d.setMenuName(name);
            d.setPath(value[0]);
            d.setType(2);
            SysMenuDO save = save(d);
            Method[] declaredMethods = aClass.getDeclaredMethods();
            for (Method declaredMethod : declaredMethods) {
                String buttonUrl = getButtonUrl(declaredMethod);
                if (buttonUrl != null) {
                    Operation annotation = declaredMethod.getAnnotation(Operation.class);
                    if (annotation != null) {
                        String path = value[0] + buttonUrl;
                        System.out.println("--按钮名：" + annotation.summary() + ";路径：" + path);
                        SysMenuDO sysMenuDO = new SysMenuDO();
                        sysMenuDO.setMenuName(annotation.summary());
                        sysMenuDO.setPath(path);
                        sysMenuDO.setType(3);
                        sysMenuDO.setParentId(save.getMenuId());
                        save(sysMenuDO);
                    }
                }
            }
        }


    }

    public SysMenuDO save(SysMenuDO sysMenuDO) {
        SysMenuDO menuDO = sysMenuMapper.selectByPathCreate(sysMenuDO);
        if (menuDO != null) {
            menuDO.setDeleted(false);
            sysMenuMapper.unDelete(menuDO);
            return menuDO;
        } else {
            sysMenuDO.setCreateUserId(-1L);
            sysMenuMapper.insert(sysMenuDO);
            return sysMenuDO;
        }
    }

    private static String getButtonUrl(Method declaredMethod) {
        RequestMapping mapping = declaredMethod.getAnnotation(RequestMapping.class);
        String buttonUrl = null;
        if (mapping != null) {
            buttonUrl = mapping.value()[0];
        } else {
            GetMapping getMapping = declaredMethod.getAnnotation(GetMapping.class);
            if (getMapping != null) {
                buttonUrl = getMapping.value()[0];

            } else {
                PostMapping postMapping = declaredMethod.getAnnotation(PostMapping.class);
                if (postMapping != null) {
                    buttonUrl = postMapping.value()[0];
                }
            }
        }
        return buttonUrl;
    }

}
