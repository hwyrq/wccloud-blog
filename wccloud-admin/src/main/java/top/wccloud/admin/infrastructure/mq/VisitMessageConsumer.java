package top.wccloud.admin.infrastructure.mq;

import cn.hutool.core.util.ArrayUtil;
import cn.hutool.core.util.StrUtil;
import cn.hutool.json.JSONUtil;
import com.rabbitmq.client.Channel;
import jakarta.annotation.PostConstruct;
import jakarta.annotation.PreDestroy;
import jakarta.annotation.Resource;
import java.io.IOException;
import lombok.extern.slf4j.Slf4j;
import org.lionsoul.ip2region.xdb.Searcher;
import org.springframework.amqp.core.Message;
import org.springframework.amqp.rabbit.annotation.Exchange;
import org.springframework.amqp.rabbit.annotation.Queue;
import org.springframework.amqp.rabbit.annotation.QueueBinding;
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.stereotype.Component;
import top.wccloud.admin.infrastructure.dao.entity.SysVisitDO;
import top.wccloud.admin.infrastructure.dao.mapper.SysVisitMapper;

/**
 * 访问日志消息消费器
 *
 * @author wcz
 */
@Slf4j
@Component
public class VisitMessageConsumer {

    @Resource
    private SysVisitMapper sysVisitMapper;

    @Resource
    private byte[] ipBytes;

    private Searcher searcher;

    @PostConstruct
    public void init() {
        try {
            // IP解析器初始化在应用启动时执行，避免每次消费消息时重复创建
            searcher = Searcher.newWithBuffer(ipBytes);
            log.info("VisitMessageConsumer IP解析器初始化成功");
        } catch (Exception e) {
            log.error("IP解析器初始化失败", e);
            throw new RuntimeException("IP解析器初始化失败", e);
        }
    }

    /**
     * 消费访问日志消息
     *
     * @param message 消息体
     * @param channel 信道
     */
    @RabbitListener(
        bindings = @QueueBinding(
            value = @Queue(
                value = "${mq.visit.queue}",
                autoDelete = "false",
                durable = "true"
            ),
            exchange = @Exchange(
                value = "${mq.visit.exchange}",
                autoDelete = "false",
                durable = "true"
            ),
            key = "${mq.visit.key}"
        ),
        ackMode = "AUTO"
    )
    public void receive(Message message, Channel channel) {
        long startTime = System.currentTimeMillis();
        String messageBody = null;

        try {
            messageBody = new String(message.getBody());
            log.debug("收到访问消息: {}", messageBody);

            SysVisitDO sysVisitDO = JSONUtil.toBean(
                messageBody,
                SysVisitDO.class
            );
            if (sysVisitDO == null || StrUtil.isBlank(sysVisitDO.getIp())) {
                log.warn("无效的访问数据，跳过处理: {}", messageBody);
                return;
            }

            // 统一设置更新时间
            sysVisitDO.setUpdateTime(sysVisitDO.getCreateTime());

            // IP地址解析与归属地填充
            fillLocationInfo(sysVisitDO);

            // 持久化访问记录
            sysVisitMapper.insert(sysVisitDO);

            log.debug(
                "访问记录插入成功，耗时: {}ms",
                System.currentTimeMillis() - startTime
            );
        } catch (ClassCastException e) {
            log.error("消息JSON格式无效: {}", messageBody, e);
        } catch (Exception e) {
            log.error(
                "处理访问消息失败: {}",
                messageBody != null ? messageBody : "null",
                e
            );
            // 由于ackMode=AUTO，异常不会导致消息重试，实际生产环境建议改为MANUAL并手动ACK/NACK
        }
    }

    /**
     * 根据IP地址填充地理位置信息
     *
     * @param sysVisitDO 访问记录
     */
    private void fillLocationInfo(SysVisitDO sysVisitDO) {
        if (searcher == null) {
            log.warn("IP解析器未初始化，跳过位置解析");
            return;
        }

        String ip = sysVisitDO.getIp();
        String address;

        try {
            address = searcher.search(ip);
            if (log.isDebugEnabled()) {
                log.debug("IP: {} -> 地址: {}", ip, address);
            }
        } catch (Exception e) {
            log.error("IP查询失败: {}", ip, e);
            return;
        }

        // 解析地址信息 [国家|区域|省份|城市|运营商]
        if (StrUtil.isBlank(address)) {
            log.debug("未找到IP对应的地址: {}", ip);
            return;
        }

        String[] segments = address.replaceAll("0", "").split("\\|", -1);

        if (log.isTraceEnabled()) {
            log.trace(
                "IP解析分段: {} -> {}",
                address,
                ArrayUtil.toString(segments)
            );
        }

        // 安全的赋值逻辑，防止数组越界
        if (segments.length > 0) sysVisitDO.setCountry(segments[0]);
        if (segments.length > 1) sysVisitDO.setRegion(segments[1]);
        if (segments.length > 2) sysVisitDO.setProvince(segments[2]);
        if (segments.length > 3) sysVisitDO.setCity(segments[3]);
        if (segments.length > 4) sysVisitDO.setIsp(segments[4]);
    }

    /**
     * 资源清理，在容器销毁时调用
     */
    @PreDestroy
    public void destroy() {
        try {
            if (searcher != null) {
                searcher.close();
                log.info("IP解析器已关闭");
            }
        } catch (Exception e) {
            log.error("关闭IP解析器时出错", e);
        }
    }
}
