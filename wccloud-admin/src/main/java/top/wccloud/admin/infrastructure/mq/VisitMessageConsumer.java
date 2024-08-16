package top.wccloud.admin.infrastructure.mq;

import cn.hutool.json.JSONUtil;
import com.rabbitmq.client.AMQP;
import com.rabbitmq.client.Channel;
import com.rabbitmq.client.impl.AMQImpl;
import io.lettuce.core.dynamic.annotation.Command;
import jakarta.annotation.Resource;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.springframework.amqp.core.Message;
import org.springframework.amqp.rabbit.annotation.Exchange;
import org.springframework.amqp.rabbit.annotation.Queue;
import org.springframework.amqp.rabbit.annotation.QueueBinding;
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;
import top.wccloud.admin.infrastructure.dao.entity.SysVisitDO;
import top.wccloud.admin.infrastructure.dao.mapper.SysVisitMapper;

/**
 * @author wcz
 */
@Component
@Slf4j
public class VisitMessageConsumer {

    @Resource
    private SysVisitMapper sysVisitMapper;


    @SneakyThrows
    @RabbitListener(bindings = {
            @QueueBinding(
                    value = @Queue(value = "${mq.visit.queue}", autoDelete = "false",durable = "true"),
                    exchange = @Exchange(value = "${mq.visit.exchange}", autoDelete = "false",durable = "true"),
                    key = {"${mq.visit.key}"}
            )
    },ackMode = "AUTO")
    public void receive(Message message, Channel channel) {
        String str = new String(message.getBody());
        //写入访问日志
        log.info(str);
        SysVisitDO sysVisitDO = JSONUtil.toBean(str, SysVisitDO.class);
        sysVisitMapper.insert(sysVisitDO);

    }
}
