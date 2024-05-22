package top.wccloud.common;

import cn.hutool.json.JSONUtil;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.springframework.validation.ObjectError;
import org.springframework.validation.beanvalidation.SpringValidatorAdapter;
import org.springframework.web.bind.MethodArgumentNotValidException;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.RestControllerAdvice;
import org.springframework.web.servlet.resource.NoResourceFoundException;

import java.lang.reflect.Field;
import java.util.List;
/**
 * 全局异常处理
 * @author wcz
 */
@RestControllerAdvice
@Slf4j
public class ExceptionAdvice {


    @ExceptionHandler(value = Throwable.class)
    @ResponseBody
    public Result<?> all(Throwable e) {
        log.error(e.getMessage(), e);
        return Result.error(e.getMessage());
    }

    /**
     * 全局参数异常梳理
     */
    @SneakyThrows
    @ExceptionHandler(value = MethodArgumentNotValidException.class)
    public Result<?> methodArgumentNotValidException(MethodArgumentNotValidException e) {
        log.error("参数异常");
        log.error(e.getMessage());
        List<ObjectError> allErrors = e.getAllErrors();
        if (!allErrors.isEmpty()) {
            return Result.error(allErrors.getFirst().getDefaultMessage());
        }
        return Result.error("参数异常", JSONUtil.toJsonStr(e.getAllErrors()));
    }

    @ExceptionHandler(value = NoResourceFoundException.class)
    public Result<?> noResourceFoundException(NoResourceFoundException e) {
        log.error(e.getMessage());
        return Result.error("参数异常", JSONUtil.toJsonStr(e.getMessage()));
    }
}
