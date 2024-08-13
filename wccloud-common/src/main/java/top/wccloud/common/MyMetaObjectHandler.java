package top.wccloud.common;

import com.baomidou.mybatisplus.core.handlers.MetaObjectHandler;
import lombok.extern.slf4j.Slf4j;
import org.apache.ibatis.reflection.MetaObject;
import org.springframework.stereotype.Component;

import java.lang.reflect.Field;
import java.time.LocalDateTime;
import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;

/**
 * mybatis自动注入字段
 * @author wcz
 */
@Component
@Slf4j
public class MyMetaObjectHandler implements MetaObjectHandler {

    public static Set<String> declaredFields(MetaObject metaObject) {
        Field[] declaredFields = metaObject.getOriginalObject().getClass().getDeclaredFields();
        Field[] declaredFields1 = metaObject.getOriginalObject().getClass().getSuperclass().getDeclaredFields();
        Set<String> collect = Arrays.stream(declaredFields).map(Field::getName).collect(Collectors.toSet());
        Set<String> collect1 = Arrays.stream(declaredFields1).map(Field::getName).collect(Collectors.toSet());
        collect.addAll(collect1);
        return collect;
    }

    @Override
    public void insertFill(MetaObject metaObject) {
        //先判断是否有该字段， 再添加值
        Set<String> set = declaredFields(metaObject);

        if (set.contains("createTime")) {
            this.strictInsertFill(metaObject, "createTime", LocalDateTime::now, LocalDateTime.class);
        }
        if (set.contains("updateTime")) {
            this.strictInsertFill(metaObject, "updateTime", LocalDateTime::now, LocalDateTime.class);
        }
        if (set.contains("createUserId")) {
            this.strictInsertFill(metaObject, "createUserId", () -> 0L, Long.class);

        }
        if (set.contains("updateUserId")) {
            this.strictInsertFill(metaObject, "updateUserId", () -> 0L, Long.class);

        }

    }

    @Override
    public void updateFill(MetaObject metaObject) {
        Set<String> set = declaredFields(metaObject);
        if (set.contains("updateTime")) {
            this.strictUpdateFill(metaObject, "updateTime", LocalDateTime::now, LocalDateTime.class);
        }
        if (set.contains("updateUserId")) {
            this.strictUpdateFill(metaObject, "updateUserId", () -> 0L, Long.class);
        }
    }
}
