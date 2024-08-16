package top.wccloud.admin.infrastructure.dao.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableLogic;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;
import lombok.EqualsAndHashCode;
import top.wccloud.common.BaseDO;

import java.io.Serializable;

/**
 * @author wcz
 */
@EqualsAndHashCode(callSuper = true)
@Data
@TableName("sys_visit")
public class SysVisitDO extends BaseDO {
    @TableId
    private Long visitId;//   bigint            not null comment '访问ID' primary key,
    private Long userId;//    bigint            not null comment '用户ID',
    private String path;//       varchar(128)      not null comment '路径',
    private String ip;//         varchar(32)       not null comment 'ip',
    private String userAgent;// varchar(256)      not null comment 'user_agent',
    private String referer;// varchar(256)      not null comment 'user_agent',
    private String host;// varchar(256)      not null comment 'user_agent',
}
