package top.wccloud.common;

import lombok.Data;

import java.io.Serializable;
/**
 * 全局mvc返回bean
 * @author wcz
 */
@Data
public class Result<T> implements Serializable {
    private int code;
    private String msg;

    private T data;

    public Result(int code, String msg, T data) {
        this.code = code;
        this.msg = msg;
        this.data = data;
    }

    public static Result<?> success() {
        return new Result<>(0, null, null);
    }

    public static Result<?> success(String msg) {
        return new Result<>(0, msg, null);
    }

    public static <T> Result<T> success(String msg, T data) {
        return new Result<>(0, msg, data);
    }

    public static Result<?> error(String msg) {
        return new Result<>(-1, msg, null);
    }

    public static <T> Result<T> error(String msg, T data) {
        return new Result<>(-1, msg, data);
    }
}
