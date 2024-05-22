package top.wccloud.gateway;

import cn.hutool.json.JSONUtil;
import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.web.reactive.error.ErrorWebExceptionHandler;
import org.springframework.core.io.buffer.DefaultDataBufferFactory;
import org.springframework.stereotype.Component;
import org.springframework.web.server.ServerWebExchange;
import reactor.core.publisher.Mono;

import java.nio.charset.StandardCharsets;
/**
 * 网关异常处理
 * @author wcz
 */
@Slf4j
@Component
public class GateWayExceptionAdvice implements ErrorWebExceptionHandler {

    @Override
    public Mono<Void> handle(ServerWebExchange exchange, Throwable ex) {
        String message = ex.getMessage();
        if (!(ex instanceof ServiceException)) {
            Result<?> error = Result.error(ex.getMessage());
            message = JSONUtil.toJsonStr(error);
        }
        String finalMessage = message;
        log.info(finalMessage);
        return exchange.getResponse().writeWith(Mono.fromSupplier(() -> new DefaultDataBufferFactory()
                .wrap(finalMessage.getBytes(StandardCharsets.UTF_8))));

    }
}
