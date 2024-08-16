-- MySQL dump 10.13  Distrib 8.3.0, for Linux (x86_64)
--
-- Host: 127.0.0.1    Database: wccloud_blog
-- ------------------------------------------------------
-- Server version	8.3.0

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `auth_user`
--

DROP TABLE IF EXISTS `auth_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `auth_user` (
  `user_id` bigint NOT NULL COMMENT '用户ID',
  `username` varchar(32) NOT NULL COMMENT '用户账号',
  `password` varchar(100) NOT NULL DEFAULT '' COMMENT '密码',
  `nickname` varchar(30) NOT NULL COMMENT '用户昵称',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `email` varchar(50) DEFAULT '' COMMENT '用户邮箱',
  `mobile` varchar(11) DEFAULT '' COMMENT '手机号码',
  `sex` tinyint DEFAULT '0' COMMENT '用户性别',
  `avatar` varchar(512) DEFAULT '' COMMENT '头像地址',
  `status` tinyint NOT NULL DEFAULT '0' COMMENT '帐号状态（0正常 1停用）',
  `login_ip` varchar(50) DEFAULT '' COMMENT '最后登录IP',
  `login_time` datetime DEFAULT NULL COMMENT '最后登录时间',
  `create_user_id` bigint DEFAULT NULL COMMENT '创建者',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user_id` bigint NOT NULL COMMENT '更新者',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `deleted` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否删除',
  PRIMARY KEY (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='用户信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `auth_user`
--

LOCK TABLES `auth_user` WRITE;
/*!40000 ALTER TABLE `auth_user` DISABLE KEYS */;
INSERT INTO `auth_user` VALUES (1781630083285852161,'admin','$2a$10$ElxUzgTy4LsZIOmALlWQQeHC9XzFAw1LqZyHy0fyqLOdfarnrqH8G','王','','','',0,'',0,'',NULL,0,'2024-04-20 18:24:48',0,'2024-05-27 11:13:35',0);
/*!40000 ALTER TABLE `auth_user` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_menu`
--

DROP TABLE IF EXISTS `sys_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_menu` (
  `menu_id` bigint NOT NULL COMMENT '菜单ID',
  `menu_name` varchar(50) NOT NULL COMMENT '菜单名称',
  `permission` varchar(100) NOT NULL DEFAULT '' COMMENT '权限标识',
  `type` tinyint NOT NULL COMMENT '菜单类型，1文件夹，2菜单，3按钮',
  `sort` int NOT NULL DEFAULT '0' COMMENT '显示顺序',
  `parent_id` bigint NOT NULL DEFAULT '0' COMMENT '父菜单ID',
  `path` varchar(200) DEFAULT '' COMMENT '路由地址',
  `icon` varchar(100) DEFAULT '' COMMENT '菜单图标',
  `status` tinyint NOT NULL DEFAULT '0' COMMENT '菜单状态，0开启，1关闭',
  `visible` tinyint NOT NULL DEFAULT '1' COMMENT '是否可见',
  `create_user_id` bigint NOT NULL COMMENT '创建者',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user_id` bigint NOT NULL COMMENT '更新者',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `deleted` tinyint NOT NULL DEFAULT '0' COMMENT '是否删除',
  PRIMARY KEY (`menu_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='菜单权限表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_menu`
--

LOCK TABLES `sys_menu` WRITE;
/*!40000 ALTER TABLE `sys_menu` DISABLE KEYS */;
INSERT INTO `sys_menu` VALUES (1,'首页','',2,0,0,'/home','',0,1,0,'2024-04-30 11:00:40',0,'2024-05-03 11:48:25',0),(1781732040449105921,'认证','',2,0,0,'/auth','',0,1,-1,'2024-04-21 01:09:56',0,'2024-05-27 15:19:37',0),(1781732040449105922,'注册','',3,0,1781732040449105921,'/auth/register','',0,1,-1,'2024-04-21 01:09:56',0,'2024-05-27 15:19:37',0),(1781732383471845378,'登录','',3,0,1781732040449105921,'/auth/login','',0,1,-1,'2024-04-21 01:11:18',0,'2024-05-27 15:19:37',0),(1781882768681590786,'菜单','',2,0,0,'/menu','',0,1,-1,'2024-04-21 11:08:52',0,'2024-05-27 15:19:37',0),(1781882768681590787,'查询所有开启菜单','',3,0,1781882768681590786,'/menu/list-status0','',0,1,-1,'2024-04-21 11:08:52',0,'2024-05-27 15:19:37',0),(1784119048509263874,'登出','',3,0,1781732040449105921,'/auth/logout','',0,1,-1,'2024-04-27 15:15:03',0,'2024-05-27 15:19:37',0),(1784127271878459394,'查询所有菜单','',3,0,1781882768681590786,'/menu/list','',0,1,-1,'2024-04-27 15:47:44',0,'2024-05-27 15:19:37',0),(1784761901624856577,'保存菜单','',3,0,1781882768681590786,'/menu/save','',0,1,-1,'2024-04-29 09:49:31',0,'2024-05-27 15:19:37',0),(1784773147153711105,'菜单11','',2,0,1784773101217693697,'/test2','',0,1,0,'2024-04-29 10:34:12',0,'2024-04-30 10:23:20',0),(1784773188878647298,'12','',2,0,1784773101217693697,'','',0,1,0,'2024-04-29 10:34:22',0,'2024-04-29 10:34:22',0),(1784775057717903361,'111','',1,0,1784773101217693697,'','',0,1,0,'2024-04-29 10:41:48',0,'2024-04-29 16:21:03',0),(1784861867466866690,'111','',1,0,1784775057717903361,'','',0,1,0,'2024-04-29 16:26:45',0,'2024-04-29 16:27:11',0),(1785224971845480450,'2121','',2,0,1784773101217693697,'','',0,1,0,'2024-04-30 16:29:36',0,'2024-04-30 16:29:36',0),(1785225003529252865,'121212','',1,0,1784773101217693697,'','',0,1,0,'2024-04-30 16:29:43',0,'2024-04-30 16:29:43',0),(1785225055366656002,'3232','',2,0,1784773101217693697,'','',0,1,0,'2024-04-30 16:29:56',0,'2024-04-30 16:29:56',0),(1785225209423441922,'sdfasd','',1,0,1784773101217693697,'','',0,1,0,'2024-04-30 16:30:32',0,'2024-04-30 16:30:32',0),(1785225240524206081,'dfg','',1,0,1784773101217693697,'','',0,1,0,'2024-04-30 16:30:40',0,'2024-04-30 16:30:40',0),(1785225274028306434,'dfgdfg','',1,0,1784773101217693697,'','',0,1,0,'2024-04-30 16:30:48',0,'2024-04-30 16:30:48',0),(1785231979604717570,'1212','',1,0,1784861867466866690,'','',0,1,0,'2024-04-30 16:57:27',0,'2024-04-30 16:57:27',0),(1785232036848578562,'43434','',1,0,1785231979604717570,'','',0,1,0,'2024-04-30 16:57:40',0,'2024-04-30 16:57:40',0),(1785864620295479297,'111','',1,0,0,'','',0,1,0,'2024-05-02 10:51:20',0,'2024-05-13 14:00:11',1),(1785864655552798722,'222','',1,0,1785864620295479297,'','',0,1,0,'2024-05-02 10:51:28',0,'2024-05-02 10:51:28',0),(1786272352970420226,'编辑','',2,0,0,'/edit','',0,1,0,'2024-05-03 13:51:31',0,'2024-05-10 17:12:37',1),(1788772965576454145,'博客','',2,0,0,'/blog','',0,1,0,'2024-05-10 11:28:03',0,'2024-05-11 21:09:06',0),(1794979419320811522,'重置密码','',3,0,1781732040449105921,'/auth/reset-pwd','',0,1,-1,'2024-05-27 14:30:17',0,'2024-05-27 15:19:37',0);
/*!40000 ALTER TABLE `sys_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `web_blog`
--

DROP TABLE IF EXISTS `web_blog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `web_blog` (
  `blog_id` bigint NOT NULL COMMENT '博客ID',
  `title` varchar(256) NOT NULL COMMENT '标题',
  `summary` varchar(2000) NOT NULL COMMENT '简介',
  `type_id` bigint NOT NULL COMMENT '分类',
  `level` tinyint NOT NULL COMMENT '推荐等级',
  `enable_comment` tinyint NOT NULL COMMENT '开启评论',
  `status` tinyint NOT NULL COMMENT '状态,0发布，1下架',
  `html` longtext NOT NULL COMMENT 'html文本',
  `md` longtext NOT NULL COMMENT 'markdown文本',
  `img_url` varchar(256) NOT NULL COMMENT '封面图片',
  `create_user_id` bigint NOT NULL COMMENT '创建者',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user_id` bigint NOT NULL COMMENT '更新者',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `deleted` tinyint NOT NULL DEFAULT '0' COMMENT '是否删除',
  PRIMARY KEY (`blog_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='博客';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `web_blog`
--

LOCK TABLES `web_blog` WRITE;
/*!40000 ALTER TABLE `web_blog` DISABLE KEYS */;
/*!40000 ALTER TABLE `web_blog` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `web_blog_label`
--

DROP TABLE IF EXISTS `web_blog_label`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `web_blog_label` (
  `blog_id` bigint NOT NULL COMMENT '博客ID',
  `label_id` bigint NOT NULL COMMENT '标签ID',
  PRIMARY KEY (`label_id`,`blog_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='博客标签';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `web_blog_label`
--

LOCK TABLES `web_blog_label` WRITE;
/*!40000 ALTER TABLE `web_blog_label` DISABLE KEYS */;
INSERT INTO `web_blog_label` VALUES (1238779418994805882,1238779420555083797),(1238779529967700094,1238779420555083797),(1238780029735797013,1238779420555083797),(1238782547790398556,1238782549325514093),(1238782547790398556,1238782550361506146),(1239452862115741920,1238782550361506146),(1241711336723841871,1238782550361506146),(1243478575298904750,1238782550361506146),(1238782547790398556,1238841660897495547),(1238843020606963998,1238843020644714640),(1238850033550363065,1238850033583918892),(1238780029735797013,1239411738604995825),(1239422581082164355,1239422582176877792),(1239422859034496501,1239422860074682609),(1239423166653138501,1239423168179864541),(1239453122351335531,1239453123886452270),(1239422581082164355,1239458343509034095),(1239494430872831971,1239494510019349445),(1241706885829099686,1241706885862657290),(1244641226674670814,1244641226930522553),(1244641398423028498,1244641226930522553);
/*!40000 ALTER TABLE `web_blog_label` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `web_label`
--

DROP TABLE IF EXISTS `web_label`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `web_label` (
  `label_id` bigint NOT NULL COMMENT '标签ID',
  `label_name` varchar(64) NOT NULL COMMENT '标签名称',
  `create_user_id` bigint NOT NULL COMMENT '创建者',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user_id` bigint NOT NULL COMMENT '更新者',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `deleted` tinyint NOT NULL DEFAULT '0' COMMENT '是否删除',
  PRIMARY KEY (`label_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='标签';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `web_label`
--

LOCK TABLES `web_label` WRITE;
/*!40000 ALTER TABLE `web_label` DISABLE KEYS */;
INSERT INTO `web_label` VALUES (1238779420555083797,'4545',0,'1970-01-01 00:00:00',0,'2024-05-13 08:18:14',1),(1238782549325514093,'linux',0,'1970-01-01 00:00:00',0,'2024-05-13 08:18:14',1),(1238782550361506146,'mysql',0,'1970-01-01 00:00:00',0,'1970-01-01 00:00:00',0),(1238841660897495547,'init',0,'1970-01-01 00:00:00',0,'2024-05-13 08:18:14',1),(1238843020644714640,'23',0,'1970-01-01 00:00:00',0,'2024-05-13 08:18:14',1),(1238850033583918892,'水电费',0,'1970-01-01 00:00:00',0,'2024-05-13 08:18:14',1),(1239411738604995825,'www',0,'1970-01-01 00:00:00',0,'2024-05-13 08:18:14',1),(1239422582176877792,'spring',0,'1970-01-01 00:00:00',0,'2024-05-19 10:58:58',0),(1239422860074682609,'oracle',0,'1970-01-01 00:00:00',0,'1970-01-01 00:00:00',0),(1239423168179864541,'cdh',0,'1970-01-01 00:00:00',0,'1970-01-01 00:00:00',0),(1239453123886452270,'docker',0,'1970-01-01 00:00:00',0,'1970-01-01 00:00:00',0),(1239458343509034095,'security',0,'1970-01-01 00:00:00',0,'1970-01-01 00:00:00',0),(1239494432626051901,'222',0,'1970-01-01 00:00:00',0,'2024-05-13 08:28:46',1),(1239494510019349445,'333',0,'1970-01-01 00:00:00',0,'2024-05-19 10:56:24',1),(1241706885862657290,'测试',0,'1970-01-01 00:00:00',0,'1970-01-01 00:00:00',0),(1244641226930522553,'1',0,'1970-01-01 00:00:00',0,'1970-01-01 00:00:00',0);
/*!40000 ALTER TABLE `web_label` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `web_type`
--

DROP TABLE IF EXISTS `web_type`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `web_type` (
  `type_id` bigint NOT NULL COMMENT '分类ID',
  `type_name` varchar(64) NOT NULL COMMENT '分类名称',
  `create_user_id` bigint NOT NULL COMMENT '创建者',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user_id` bigint NOT NULL COMMENT '更新者',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `deleted` tinyint NOT NULL DEFAULT '0' COMMENT '是否删除',
  PRIMARY KEY (`type_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='分类';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `web_type`
--

LOCK TABLES `web_type` WRITE;
/*!40000 ALTER TABLE `web_type` DISABLE KEYS */;
INSERT INTO `web_type` VALUES (1238779418994803663,'的轨顶风道',1,'1970-01-01 00:00:00',1,'2024-05-13 08:14:48',1),(1238782547790398686,'后端',1,'1970-01-01 00:00:00',1,'1970-01-01 00:00:00',0),(1238843020606964758,'222',1,'1970-01-01 00:00:00',1,'2024-05-13 08:14:48',1),(1238850033550362741,'多少度',1,'1970-01-01 00:00:00',1,'2024-05-13 08:14:48',1),(1239423166653140948,'大数据',1,'1970-01-01 00:00:00',1,'1970-01-01 00:00:00',0),(1239453122351335930,'运维',1,'1970-01-01 00:00:00',1,'1970-01-01 00:00:00',0),(1239494430872831757,'111',1,'1970-01-01 00:00:00',1,'2024-05-13 08:28:40',1),(1239494507485989598,'333',1,'1970-01-01 00:00:00',1,'2024-05-19 10:56:24',1),(1241706885829102860,'测试',1,'1970-01-01 00:00:00',1,'1970-01-01 00:00:00',0),(1244641226674670792,'1',1,'1970-01-01 00:00:00',1,'1970-01-01 00:00:00',0);
/*!40000 ALTER TABLE `web_type` ENABLE KEYS */;

DROP TABLE IF EXISTS `sys_visit`;

create table sys_visit
(
    visit_id       bigint                             not null comment '访问ID'
        primary key,
    user_id        bigint                             not null comment '用户ID',
    path           varchar(128)                       not null comment '路径',
    ip             varchar(32)                        not null comment 'ip',
    referer        varchar(64)                        null,
    host           varchar(64)                        null,
    user_agent     varchar(256)                       not null comment 'user_agent',
    create_user_id bigint                             not null comment '创建者',
    create_time    datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_user_id bigint                             not null comment '更新者',
    update_time    datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '更新时间',
    deleted        tinyint  default 0                 not null comment '是否删除'
)
    comment '访问记录';




UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2024-05-27 21:59:39
