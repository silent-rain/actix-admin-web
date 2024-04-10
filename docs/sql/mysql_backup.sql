-- MySQL dump 10.19  Distrib 10.3.36-MariaDB, for debian-linux-gnu (x86_64)
--
-- Host: localhost    Database: gin_admin
-- ------------------------------------------------------
-- Server version	10.3.36-MariaDB-0+deb10u2

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `api_http`
--

DROP TABLE IF EXISTS `api_http`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `api_http` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `parent_id` int(20) DEFAULT NULL COMMENT '父接口ID',
  `name` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '接口名称',
  `method` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '请求类型',
  `uri` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'URI资源',
  `note` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '备注',
  `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用,0:禁用,1:启用',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='Http协议接口管理表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `api_http`
--

LOCK TABLES `api_http` WRITE;
/*!40000 ALTER TABLE `api_http` DISABLE KEYS */;
INSERT INTO `api_http` VALUES (1,NULL,'xxx','GET','xx','',1,'2023-03-11 15:25:47','2023-03-11 15:25:47'),(2,NULL,'a','GET','sa','',1,'2023-03-11 15:25:56','2023-03-11 15:25:56'),(3,2,'as','POST','as','',1,'2023-03-11 15:26:03','2023-03-11 16:10:19'),(4,2,'asxx','GET','asxx','',1,'2023-03-11 15:27:32','2023-03-11 15:27:32'),(6,1,'asd','POST','asd','',1,'2023-03-11 15:35:24','2023-03-11 15:35:24');
/*!40000 ALTER TABLE `api_http` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `api_role_http_rel`
--

DROP TABLE IF EXISTS `api_role_http_rel`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `api_role_http_rel` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `role_id` int(11) NOT NULL COMMENT '角色ID',
  `api_id` int(11) NOT NULL COMMENT 'Http协议接口ID',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `api_role_http_rel_role_id` (`role_id`),
  KEY `api_role_http_rel_api_id` (`api_id`),
  CONSTRAINT `api_role_http_rel_api_id` FOREIGN KEY (`api_id`) REFERENCES `api_http` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `api_role_http_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=20 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='角色与Http协议接口关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `api_role_http_rel`
--

LOCK TABLES `api_role_http_rel` WRITE;
/*!40000 ALTER TABLE `api_role_http_rel` DISABLE KEYS */;
INSERT INTO `api_role_http_rel` VALUES (6,3,1,'2023-03-11 15:48:30','2023-03-11 15:48:30'),(7,3,6,'2023-03-11 15:48:30','2023-03-11 15:48:30'),(8,11,1,'2023-03-11 16:00:50','2023-03-11 16:00:50'),(9,11,6,'2023-03-11 16:00:50','2023-03-11 16:00:50'),(10,11,2,'2023-03-11 16:01:28','2023-03-11 16:01:28'),(11,11,3,'2023-03-11 16:01:28','2023-03-11 16:01:28'),(12,11,4,'2023-03-11 16:01:28','2023-03-11 16:01:28'),(13,14,1,'2023-03-11 16:06:20','2023-03-11 16:06:20'),(14,14,6,'2023-03-11 16:06:20','2023-03-11 16:06:20'),(15,1,1,'2023-03-11 16:17:08','2023-03-11 16:17:08'),(16,1,6,'2023-03-11 16:17:08','2023-03-11 16:17:08'),(17,1,2,'2023-03-11 16:17:08','2023-03-11 16:17:08'),(18,1,4,'2023-03-11 16:17:08','2023-03-11 16:17:08'),(19,1,3,'2023-03-11 16:17:08','2023-03-11 16:17:08');
/*!40000 ALTER TABLE `api_role_http_rel` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `api_token`
--

DROP TABLE IF EXISTS `api_token`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `api_token` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `user_id` int(20) NOT NULL COMMENT '用户ID',
  `nickname` varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '昵称',
  `token` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Token信息',
  `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用,0:禁用,1:启用',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uni_api_token_user_id` (`user_id`),
  CONSTRAINT `api_token_user_id` FOREIGN KEY (`user_id`) REFERENCES `perm_user` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='API_Token令牌表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `api_token`
--

LOCK TABLES `api_token` WRITE;
/*!40000 ALTER TABLE `api_token` DISABLE KEYS */;
/*!40000 ALTER TABLE `api_token` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `log_http`
--

DROP TABLE IF EXISTS `log_http`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `log_http` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `user_id` int(11) DEFAULT NULL COMMENT '请求用户ID',
  `nickname` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `trace_id` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '上游请求traceId',
  `status_code` int(10) NOT NULL COMMENT '请求状态码',
  `method` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '请求方法',
  `path` varchar(500) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '请求地址路径',
  `query` varchar(500) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '请求参数',
  `body` longtext COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '请求体/响应体',
  `remote_addr` varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '请求IP',
  `user_agent` varchar(256) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '用户代理',
  `cost` int(20) NOT NULL COMMENT '耗时,纳秒',
  `htpp_type` varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '日志类型:REQ/RSP',
  `note` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '备注',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='网络请求日志';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `log_http`
--

LOCK TABLES `log_http` WRITE;
/*!40000 ALTER TABLE `log_http` DISABLE KEYS */;
/*!40000 ALTER TABLE `log_http` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `log_system`
--

DROP TABLE IF EXISTS `log_system`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `log_system` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `user_id` int(11) DEFAULT NULL COMMENT '请求用户ID',
  `nickname` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `trace_id` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '请求traceId',
  `span_id` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '埋点spanId',
  `level` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '日志级别',
  `caller_line` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '日志发生位置',
  `error_code` int(10) DEFAULT NULL COMMENT '业务错误码',
  `error_msg` varchar(500) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '业务错误信息',
  `msg` text COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '日志消息',
  `stack` text COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `extend` text COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '日志扩展信息/json',
  `note` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '备注',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='系统日志';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `log_system`
--

LOCK TABLES `log_system` WRITE;
/*!40000 ALTER TABLE `log_system` DISABLE KEYS */;
/*!40000 ALTER TABLE `log_system` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `log_web`
--

DROP TABLE IF EXISTS `log_web`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `log_web` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `user_id` int(11) DEFAULT NULL COMMENT '请求用户ID',
  `nickname` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '昵称',
  `trace_id` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '请求traceId',
  `os_type` tinyint(2) NOT NULL COMMENT '终端类型: 0: 未知,1: 安卓,2 :ios,3 :web',
  `error_type` tinyint(2) NOT NULL COMMENT '错误类型: 1:接口报错,2:代码报错',
  `level` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT ' 日志级别 ',
  `caller_line` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT ' 日发生位置 ',
  `url` varchar(500) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT ' 错误页面 ',
  `msg` text COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT ' 日志消息 ',
  `stack` text COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT ' 堆栈信息 ',
  `note` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT ' 备注 ',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT ' 创建时间 ',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT=' WEB日志表 ';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `log_web`
--

LOCK TABLES `log_web` WRITE;
/*!40000 ALTER TABLE `log_web` DISABLE KEYS */;
/*!40000 ALTER TABLE `log_web` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `perm_menu`
--

DROP TABLE IF EXISTS `perm_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `perm_menu` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '菜单ID',
  `parent_id` int(20) DEFAULT NULL COMMENT '父菜单ID',
  `title` varchar(20) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '菜单名称',
  `icon` varchar(20) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '菜单图标',
  `el_svg_icon` varchar(20) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT 'Element菜单图标',
  `menu_type` tinyint(1) NOT NULL DEFAULT 0 COMMENT '菜单类型,0:菜单,1:按钮',
  `open_type` tinyint(1) NOT NULL DEFAULT 0 COMMENT '打开方式,0:组件,1:内链,2:外链',
  `path` varchar(500) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '路由地址',
  `name` varchar(50) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '路由别名',
  `component` varchar(500) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '组件路径',
  `redirect` varchar(500) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '路由重定向',
  `link` varchar(500) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '链接地址:内链地址/外链地址',
  `target` varchar(500) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '链接地址跳转方式, component/_blank/_self',
  `permission` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '权限标识',
  `hidden` tinyint(1) DEFAULT 0 COMMENT '是否隐藏,0:显示,1:隐藏',
  `always_show` tinyint(1) DEFAULT 1 COMMENT '始终显示根菜单,0:显示,1:隐藏',
  `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
  `note` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '备注',
  `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态,0:停用,1:启用',
  `create_user_id` int(11) DEFAULT NULL COMMENT '创建菜单用户ID',
  `update_user_id` int(11) DEFAULT NULL COMMENT '更新菜单用户ID',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=75 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='菜单表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `perm_menu`
--

LOCK TABLES `perm_menu` WRITE;
/*!40000 ALTER TABLE `perm_menu` DISABLE KEYS */;
INSERT INTO `perm_menu` VALUES (1,NULL,'权限管理','','UserFilled',0,0,'/permission','','Layout','','','','',0,1,1,'',1,0,20,'2023-02-04 21:18:29','2023-03-11 00:31:22'),(2,1,'用户管理','','User',0,0,'/permission/user','用户管理','@/views/permission/user/index.vue','','','','',0,1,1,'',1,0,20,'2023-02-04 21:19:01','2023-03-05 18:28:55'),(3,1,'角色管理','','Postcard',0,0,'/permission/role','角色管理','@/views/permission/role/index.vue','','','','',0,1,2,'',1,0,20,'2023-02-04 21:21:53','2023-03-05 18:30:55'),(4,1,'菜单管理','','Grid',0,0,'/permission/menu','菜单管理','@/views/permission/menu/index.vue','','','','',0,1,4,'',1,0,20,'2023-02-04 21:22:30','2023-03-09 21:07:32'),(5,NULL,'系统管理','','Setting',0,0,'/system','系统管理','Layout','/system/websiteConfig','','','',0,1,1,'',1,0,20,'2023-02-04 22:34:07','2023-03-05 18:31:10'),(6,63,'全局配置管理','','',0,0,'/dataCenter/config','全局配置管理','@/views/data-center/config/index.vue','','','','',0,1,1,'',1,0,20,'2023-02-04 22:34:18','2023-03-10 23:52:19'),(7,2,'查询用户','','',1,0,'','','','','','','sys:user:list',0,1,1,'',1,0,20,'2023-02-05 20:29:03','2023-02-11 11:36:18'),(8,2,'添加用户','',NULL,1,0,'',NULL,'',NULL,'','','sys:user:add',0,1,1,'',1,20,20,'2023-02-05 20:29:19','2023-02-10 00:35:44'),(9,2,'修改用户','','',1,0,'','','','','','','sys:user:update',0,1,2,'',1,0,20,'2023-02-05 20:29:31','2023-02-11 11:18:27'),(10,2,'删除用户','','',1,0,'','','','','','','sys:user:delete',0,1,3,'',1,0,20,'2023-02-05 20:29:50','2023-02-11 11:18:36'),(11,2,'批量删除','','',1,0,'','','','','','','sys:user:delall',0,1,1,'',1,0,20,'2023-02-05 20:30:03','2023-02-11 11:30:35'),(12,2,'设置状态','','',1,0,'','','','','','','sys:user:status',0,1,4,'',1,0,20,'2023-02-05 20:30:16','2023-02-11 11:18:47'),(13,2,'重置密码','','',1,0,'','','','','','','sys:user:resetPwd',0,1,5,'',1,0,20,'2023-02-05 20:30:27','2023-02-11 11:19:00'),(14,2,'导出用户','',NULL,1,0,'',NULL,'',NULL,'','','sys:user:export',0,1,1,'',1,20,20,'2023-02-05 20:30:40','2023-02-11 11:20:05'),(15,NULL,'tttt1','','Pointer',0,0,'/tttt1','tttt1','Layout','','','','',0,1,2,'',1,0,20,'2023-02-10 21:47:33','2023-03-05 18:36:58'),(16,15,'ttt2','','',0,0,'/ttt2','ttt2','@/views/system/role/index.vue','','','','',0,0,1,'',1,20,20,'2023-02-10 21:48:37','2023-02-10 21:48:37'),(17,15,'ttt3','','',0,2,'https://www.baidu.com/','','','','','','',0,0,1,'',1,0,20,'2023-02-10 21:49:23','2023-03-11 00:15:34'),(18,2,'导入用户','','',1,0,'','','','','','','sys:user:import',0,1,1,'',0,20,20,'2023-02-11 11:19:48','2023-02-11 11:20:01'),(19,3,'查询角色','','',1,0,'','','','','','','sys:role:list',0,1,1,'',1,20,20,'2023-02-11 11:50:31','2023-02-11 11:50:31'),(20,3,'添加角色','','',1,0,'','','','','','','sys:role:add',0,1,1,'',1,20,20,'2023-02-11 11:50:49','2023-02-11 11:50:49'),(21,3,'修改角色','','',1,0,'','','','','','','sys:role:update',0,1,1,'',1,20,20,'2023-02-11 11:51:06','2023-02-11 11:51:06'),(22,3,'删除角色','','',1,0,'','','','','','','sys:role:delete',0,1,1,'',1,20,20,'2023-02-11 11:51:18','2023-02-11 11:51:18'),(23,3,'批量删除','','',1,0,'','','','','','','sys:role:delall',0,1,1,'',1,0,20,'2023-02-11 11:51:30','2023-02-11 11:54:22'),(24,3,'分配权限','','',1,0,'','','','','','','sys:role:permission',0,1,1,'',1,20,20,'2023-02-11 11:51:45','2023-02-11 11:51:45'),(25,3,'设置状态','','',1,0,'','','','','','','sys:role:status',0,1,1,'',1,0,20,'2023-02-11 14:22:36','2023-02-11 14:47:51'),(26,4,'查询菜单','','',1,0,'','','','','','','sys:menu:list',0,1,1,'',1,20,20,'2023-02-11 14:22:59','2023-02-11 14:22:59'),(27,4,'添加菜单','','',1,0,'','','','','','','sys:menu:add',0,1,1,'',1,20,20,'2023-02-11 14:23:12','2023-02-11 14:23:12'),(28,4,'修改菜单','','',1,0,'','','','','','','sys:menu:update',0,1,1,'',1,20,20,'2023-02-11 14:23:26','2023-02-11 14:23:26'),(29,4,'删除菜单','','',1,0,'','','','','','','sys:menu:delete',0,1,1,'',1,20,20,'2023-02-11 14:23:39','2023-02-11 14:23:39'),(30,4,'设置状态','','',1,0,'','','','','','','sys:menu:status',0,1,1,'',1,20,20,'2023-02-11 14:24:06','2023-02-11 14:24:06'),(31,4,'添加子级','','',1,0,'','','','','','','sys:menu:addchild',0,1,1,'',1,20,20,'2023-02-11 14:24:16','2023-02-11 14:24:16'),(32,4,'全部展开','','',1,0,'','','','','','','sys:menu:expand',0,1,1,'',1,20,20,'2023-02-11 14:24:40','2023-02-11 14:24:40'),(33,4,'全部折叠','','',1,0,'','','','','','','sys:menu:collapse',0,1,1,'',1,20,20,'2023-02-11 14:24:51','2023-02-11 14:24:51'),(35,6,'查询配置','','',1,0,'','','','','','','sys:config:list',0,1,1,'',1,20,20,'2023-02-14 21:49:34','2023-02-14 21:49:34'),(36,6,'添加配置','','',1,0,'','','','','','','sys:config:add',0,1,1,'',1,20,20,'2023-02-14 21:49:52','2023-02-14 21:49:52'),(37,6,'修改配置','','',1,0,'','','','','','','sys:config:update',0,1,1,'',1,20,20,'2023-02-14 21:50:01','2023-02-14 21:50:01'),(38,6,'删除配置','','',1,0,'','','','','','','sys:config:delete',0,1,1,'',1,20,20,'2023-02-14 21:50:11','2023-02-14 21:50:11'),(39,6,'设置状态','','',1,0,'','','','','','','sys:config:status',0,1,1,'',1,20,20,'2023-02-14 21:50:23','2023-02-14 21:50:23'),(40,6,'添加子级','','',1,0,'','','','','','','sys:config:addchild',0,1,1,'',1,20,20,'2023-02-14 21:50:34','2023-02-14 21:50:34'),(41,6,'全部展开','','',1,0,'','','','','','','sys:config:expand',0,1,1,'',1,20,20,'2023-02-14 21:50:44','2023-02-14 21:50:44'),(42,6,'全部折叠','','',1,0,'','','','','','','sys:config:collapse',0,1,1,'',1,20,20,'2023-02-14 21:50:53','2023-02-14 21:50:53'),(43,5,'请求日志管理','','',0,0,'/system/log/httpLog','请求日志管理','@/views/system/log/httpLog.vue','','','','',0,1,3,'',1,0,20,'2023-02-20 20:14:27','2023-02-25 17:38:44'),(44,5,'系统日志管理','','',0,0,'/system/log/systemLog','系统日志管理','@/views/system/log/systemLog.vue','','','','',0,1,4,'',1,0,20,'2023-02-20 20:20:57','2023-02-25 17:38:57'),(45,43,'查询列表','','',1,0,'','','','','','','sys:httplog:list',0,1,1,'',1,20,20,'2023-02-20 21:09:42','2023-02-20 21:09:42'),(46,44,'查询列表','','',1,0,'','','','','','','sys:systemlog:list',0,1,1,'',1,20,20,'2023-02-20 21:09:53','2023-02-20 21:09:53'),(47,17,'testsss','','',1,0,'','','','','','','xxxxxxx',1,0,4,'',1,20,20,'2023-02-20 23:54:11','2023-02-20 23:54:11'),(48,17,'xx11','xx11','xx11',0,0,'xx11','xx11','xx11','xx11','','','xx11',0,1,1,'',1,20,20,'2023-02-20 23:54:48','2023-02-20 23:54:48'),(49,17,'xxx2','xxx2','xxx2',0,1,'xxx2','xxx2','xxx2','','xxx2','','',0,1,1,'xxx2',1,0,20,'2023-02-20 23:58:23','2023-02-20 23:59:39'),(50,NULL,'用户中心','','',0,0,'/user','用户中心','Layout','','','','',1,0,1,'',1,0,20,'2023-02-24 00:12:50','2023-02-25 21:03:50'),(51,50,'个人资料','','',0,0,'/user/profile','个人资料','@/views/user/profile/profile.vue','','','','',1,0,1,'',1,0,20,'2023-02-24 00:13:33','2023-02-25 21:03:41'),(52,5,'网站配置管理','','',0,0,'/system/websiteConfig','网站配置管理','@/views/system/website-config/index.vue','','','','',0,1,2,'',1,0,20,'2023-02-25 17:37:32','2023-02-25 21:52:02'),(54,5,'WEB日志管理','','',0,0,'/system/log/webLog','WEB日志管理','@/views/system/log/webLog.vue','','','','',0,1,5,'',1,0,20,'2023-02-26 22:01:34','2023-02-26 22:02:08'),(55,54,'查询列表','','',1,0,'','','','','','','sys:weblog:list',0,1,1,'',1,20,20,'2023-02-26 22:22:11','2023-02-26 22:22:11'),(56,5,'登录管理','','',0,0,'/system/userLogin','登录管理','@/views/system/userLogin/index.vue','','','','',0,1,5,'',1,0,20,'2023-03-05 23:07:38','2023-03-05 23:07:56'),(57,1,'API令牌管理','','Guide',0,0,'/permission/userApiToken','API令牌管理','@/views/permission/user-api-token/index.vue','','','','',0,1,5,'',1,0,20,'2023-03-09 21:06:55','2023-03-11 09:48:11'),(58,57,'查询列表','','',1,0,'','','','','','','sys:userToken:list',0,1,1,'',1,20,20,'2023-03-10 23:05:25','2023-03-10 23:05:25'),(59,NULL,'接口管理','','FolderOpened',0,0,'/apiAuth','数据中心','Layout','','','','',0,1,1,'',1,0,20,'2023-03-10 23:09:31','2023-03-10 23:10:59'),(60,59,'HTTP接口管理','','',0,0,'/apiAuth/apiHttp','HTTP接口管理','@/views/api-auth/api-http/index.vue','','','','',0,1,1,'',1,0,20,'2023-03-10 23:10:13','2023-03-10 23:10:51'),(63,NULL,'数据中心','','OfficeBuilding',0,0,'/dataCenter','用户中心','Layout','','','','',0,1,1,'',1,0,20,'2023-03-10 23:49:00','2023-03-11 00:32:28'),(64,56,'查询列表','','',1,0,'','','','','','','sys:userlogin:list',0,1,1,'',1,20,20,'2023-03-10 23:58:17','2023-03-10 23:58:17'),(65,56,'设置状态','','',1,0,'','','','','','','sys:userlogin:status',0,1,1,'',1,20,20,'2023-03-10 23:59:24','2023-03-10 23:59:24'),(66,60,'查询列表','','',1,0,'','','','','','','apiAuth:apiHttp:list',0,1,1,'',1,20,20,'2023-03-11 00:02:05','2023-03-11 00:02:05'),(67,60,'添加接口','','',1,0,'','','','','','','apiAuth:apiHttp:add',0,1,1,'',1,20,20,'2023-03-11 00:02:23','2023-03-11 00:02:23'),(68,60,'修改接口','','',1,0,'','','','','','','apiAuth:apiHttp:update',0,1,1,'',1,20,20,'2023-03-11 00:02:34','2023-03-11 00:02:34'),(69,60,'删除接口','','',1,0,'','','','','','','apiAuth:apiHttp:delete',0,1,1,'',1,20,20,'2023-03-11 00:02:45','2023-03-11 00:02:45'),(70,60,'批量删除','','',1,0,'','','','','','','apiAuth:apiHttp:delall',0,1,1,'',1,20,20,'2023-03-11 00:02:57','2023-03-11 00:02:57'),(71,60,'设置状态','','',1,0,'','','','','','','apiAuth:apiHttp:status',0,1,1,'',1,20,20,'2023-03-11 00:03:14','2023-03-11 00:03:14'),(72,60,'添加子级','','',1,0,'','','','','','','apiAuth:apiHttp:addchild',0,1,1,'',1,20,20,'2023-03-11 15:18:44','2023-03-11 15:18:44'),(73,60,'全部展开','','',1,0,'','','','','','','apiAuth:apiHttp:expand',0,1,1,'',1,20,20,'2023-03-11 15:38:04','2023-03-11 15:38:04'),(74,60,'全部折叠','','',1,0,'','','','','','','apiAuth:apiHttp:collapse',0,1,1,'',1,20,20,'2023-03-11 15:38:15','2023-03-11 15:38:15');
/*!40000 ALTER TABLE `perm_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `perm_role`
--

DROP TABLE IF EXISTS `perm_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `perm_role` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '角色ID',
  `name` varchar(20) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '角色名称',
  `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
  `note` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '备注',
  `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '角色状态,0:停用,1:启用',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=15 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='角色表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `perm_role`
--

LOCK TABLES `perm_role` WRITE;
/*!40000 ALTER TABLE `perm_role` DISABLE KEYS */;
INSERT INTO `perm_role` VALUES (1,'管理员',1,'',1,'2023-02-01 22:39:38','2023-02-01 22:41:48'),(2,'普通用户',1,'',1,'2023-02-01 22:41:58','2023-02-25 14:24:40'),(3,'xxxx',1,'',1,'2023-02-01 22:42:10','2023-03-11 00:13:28'),(11,'xxx12234',1,'',1,'2023-02-01 23:13:18','2023-02-12 21:46:12'),(13,'xxx',1,'',1,'2023-02-12 21:47:03','2023-02-12 21:47:03'),(14,'test22',1,'',0,'2023-02-22 20:29:15','2023-03-05 14:00:48');
/*!40000 ALTER TABLE `perm_role` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `perm_menu_role_rel`
--

DROP TABLE IF EXISTS `perm_menu_role_rel`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `perm_menu_role_rel` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `role_id` int(10) NOT NULL COMMENT '角色ID',
  `menu_id` int(10) NOT NULL COMMENT '菜单ID',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `sys_role_menu_rel_role_id` (`role_id`),
  KEY `sys_role_menu_rel_menu_id` (`menu_id`),
  CONSTRAINT `sys_role_menu_rel_menu_id` FOREIGN KEY (`menu_id`) REFERENCES `perm_menu` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `sys_role_menu_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=222 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='菜单角色关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `perm_menu_role_rel`
--

LOCK TABLES `perm_menu_role_rel` WRITE;
/*!40000 ALTER TABLE `perm_menu_role_rel` DISABLE KEYS */;
INSERT INTO `perm_menu_role_rel` VALUES (14,3,3,'2023-02-05 17:35:44','2023-02-05 17:35:44'),(16,3,2,'2023-02-05 17:38:00','2023-02-05 17:38:00'),(17,1,1,'2023-02-05 20:08:22','2023-02-05 20:08:22'),(18,1,2,'2023-02-05 20:08:22','2023-02-05 20:08:22'),(19,1,3,'2023-02-05 20:08:22','2023-02-05 20:08:22'),(20,1,4,'2023-02-05 20:08:22','2023-02-05 20:08:22'),(22,2,4,'2023-02-05 20:08:28','2023-02-05 20:08:28'),(23,1,7,'2023-02-05 20:37:20','2023-02-05 20:37:20'),(24,1,8,'2023-02-05 20:37:20','2023-02-05 20:37:20'),(25,1,9,'2023-02-05 20:37:20','2023-02-05 20:37:20'),(26,1,10,'2023-02-05 20:37:20','2023-02-05 20:37:20'),(27,1,11,'2023-02-05 20:37:20','2023-02-05 20:37:20'),(28,1,12,'2023-02-05 20:37:20','2023-02-05 20:37:20'),(29,1,13,'2023-02-05 20:37:20','2023-02-05 20:37:20'),(30,1,14,'2023-02-05 20:37:20','2023-02-05 20:37:20'),(31,1,15,'2023-02-10 21:52:08','2023-02-10 21:52:08'),(32,1,16,'2023-02-10 21:52:08','2023-02-10 21:52:08'),(33,1,17,'2023-02-10 21:52:08','2023-02-10 21:52:08'),(38,1,5,'2023-02-10 22:07:17','2023-02-10 22:07:17'),(39,1,6,'2023-02-10 22:07:17','2023-02-10 22:07:17'),(40,1,18,'2023-02-11 11:52:40','2023-02-11 11:52:40'),(41,1,19,'2023-02-11 11:52:40','2023-02-11 11:52:40'),(42,1,20,'2023-02-11 11:52:40','2023-02-11 11:52:40'),(43,1,21,'2023-02-11 11:52:40','2023-02-11 11:52:40'),(44,1,22,'2023-02-11 11:52:40','2023-02-11 11:52:40'),(45,1,23,'2023-02-11 11:52:40','2023-02-11 11:52:40'),(46,1,24,'2023-02-11 11:52:40','2023-02-11 11:52:40'),(47,1,25,'2023-02-11 14:30:31','2023-02-11 14:30:31'),(48,1,26,'2023-02-11 14:30:31','2023-02-11 14:30:31'),(49,1,27,'2023-02-11 14:30:31','2023-02-11 14:30:31'),(50,1,28,'2023-02-11 14:30:31','2023-02-11 14:30:31'),(51,1,29,'2023-02-11 14:30:31','2023-02-11 14:30:31'),(52,1,30,'2023-02-11 14:30:31','2023-02-11 14:30:31'),(53,1,31,'2023-02-11 14:30:31','2023-02-11 14:30:31'),(54,1,32,'2023-02-11 14:30:31','2023-02-11 14:30:31'),(55,1,33,'2023-02-11 14:30:31','2023-02-11 14:30:31'),(56,2,2,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(57,2,7,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(58,2,8,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(59,2,11,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(60,2,14,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(61,2,18,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(62,2,9,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(63,2,10,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(64,2,12,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(65,2,13,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(66,2,26,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(67,2,27,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(68,2,28,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(69,2,29,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(70,2,30,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(71,2,31,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(72,2,32,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(73,2,33,'2023-02-12 21:46:41','2023-02-12 21:46:41'),(74,1,35,'2023-02-14 21:52:03','2023-02-14 21:52:03'),(75,1,36,'2023-02-14 21:52:03','2023-02-14 21:52:03'),(76,1,37,'2023-02-14 21:52:03','2023-02-14 21:52:03'),(77,1,38,'2023-02-14 21:52:03','2023-02-14 21:52:03'),(78,1,39,'2023-02-14 21:52:03','2023-02-14 21:52:03'),(79,1,40,'2023-02-14 21:52:03','2023-02-14 21:52:03'),(80,1,41,'2023-02-14 21:52:03','2023-02-14 21:52:03'),(81,1,42,'2023-02-14 21:52:03','2023-02-14 21:52:03'),(82,1,43,'2023-02-20 20:21:21','2023-02-20 20:21:21'),(83,1,44,'2023-02-20 20:21:21','2023-02-20 20:21:21'),(84,1,45,'2023-02-20 21:10:15','2023-02-20 21:10:15'),(85,1,46,'2023-02-20 21:10:15','2023-02-20 21:10:15'),(126,1,50,'2023-02-24 00:39:07','2023-02-24 00:39:07'),(127,1,51,'2023-02-24 00:39:07','2023-02-24 00:39:07'),(128,1,48,'2023-02-24 00:39:07','2023-02-24 00:39:07'),(129,1,49,'2023-02-24 00:39:07','2023-02-24 00:39:07'),(130,1,47,'2023-02-24 00:39:07','2023-02-24 00:39:07'),(131,1,52,'2023-02-25 17:37:55','2023-02-25 17:37:55'),(133,1,54,'2023-02-26 22:01:45','2023-02-26 22:01:45'),(134,1,55,'2023-02-26 22:22:28','2023-02-26 22:22:28'),(135,1,56,'2023-03-05 23:09:53','2023-03-05 23:09:53'),(136,1,57,'2023-03-09 21:07:45','2023-03-09 21:07:45'),(137,1,58,'2023-03-10 23:06:20','2023-03-10 23:06:20'),(138,1,59,'2023-03-10 23:11:13','2023-03-10 23:11:13'),(139,1,60,'2023-03-10 23:11:13','2023-03-10 23:11:13'),(140,1,63,'2023-03-10 23:52:48','2023-03-10 23:52:48'),(141,1,64,'2023-03-10 23:59:42','2023-03-10 23:59:42'),(142,1,65,'2023-03-10 23:59:42','2023-03-10 23:59:42'),(143,1,66,'2023-03-11 00:03:51','2023-03-11 00:03:51'),(144,1,67,'2023-03-11 00:03:51','2023-03-11 00:03:51'),(145,1,68,'2023-03-11 00:03:51','2023-03-11 00:03:51'),(146,1,69,'2023-03-11 00:03:51','2023-03-11 00:03:51'),(147,1,70,'2023-03-11 00:03:51','2023-03-11 00:03:51'),(148,1,71,'2023-03-11 00:03:51','2023-03-11 00:03:51'),(149,13,2,'2023-03-11 11:38:42','2023-03-11 11:38:42'),(150,13,7,'2023-03-11 11:38:42','2023-03-11 11:38:42'),(151,13,8,'2023-03-11 11:38:42','2023-03-11 11:38:42'),(152,13,11,'2023-03-11 11:38:42','2023-03-11 11:38:42'),(153,13,14,'2023-03-11 11:38:42','2023-03-11 11:38:42'),(154,13,18,'2023-03-11 11:38:42','2023-03-11 11:38:42'),(155,13,9,'2023-03-11 11:38:42','2023-03-11 11:38:42'),(156,13,10,'2023-03-11 11:38:42','2023-03-11 11:38:42'),(157,13,12,'2023-03-11 11:38:42','2023-03-11 11:38:42'),(158,13,13,'2023-03-11 11:38:42','2023-03-11 11:38:42'),(159,14,1,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(160,14,2,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(161,14,7,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(162,14,8,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(163,14,11,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(164,14,14,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(165,14,18,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(166,14,9,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(167,14,10,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(168,14,12,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(169,14,13,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(170,14,3,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(171,14,19,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(172,14,20,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(173,14,21,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(174,14,22,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(175,14,23,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(176,14,24,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(177,14,25,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(178,14,4,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(179,14,26,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(180,14,27,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(181,14,28,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(182,14,29,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(183,14,30,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(184,14,31,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(185,14,32,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(186,14,33,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(187,14,57,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(188,14,58,'2023-03-11 11:38:55','2023-03-11 11:38:55'),(189,1,72,'2023-03-11 15:18:59','2023-03-11 15:18:59'),(190,1,73,'2023-03-11 15:38:31','2023-03-11 15:38:31'),(191,1,74,'2023-03-11 15:38:31','2023-03-11 15:38:31'),(192,11,1,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(193,11,2,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(194,11,7,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(195,11,8,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(196,11,11,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(197,11,14,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(198,11,18,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(199,11,9,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(200,11,10,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(201,11,12,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(202,11,13,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(203,11,3,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(204,11,19,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(205,11,20,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(206,11,21,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(207,11,22,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(208,11,23,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(209,11,24,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(210,11,25,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(211,11,4,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(212,11,26,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(213,11,27,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(214,11,28,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(215,11,29,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(216,11,30,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(217,11,31,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(218,11,32,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(219,11,33,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(220,11,57,'2023-03-11 16:01:21','2023-03-11 16:01:21'),(221,11,58,'2023-03-11 16:01:21','2023-03-11 16:01:21');
/*!40000 ALTER TABLE `perm_menu_role_rel` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `perm_user`
--

DROP TABLE IF EXISTS `perm_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `perm_user` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '用户ID',
  `realname` varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '真实姓名',
  `nickname` varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '昵称',
  `gender` tinyint(1) NOT NULL COMMENT '性别: 0:女,1:男',
  `age` int(11) DEFAULT NULL COMMENT '年龄',
  `birthday` varchar(20) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '出生日期',
  `avatar` varchar(100) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '用户头像URL',
  `phone` varchar(20) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '手机号码',
  `email` varchar(50) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '邮件',
  `intro` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '介绍',
  `note` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '备注',
  `password` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '密码',
  `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
  `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用,0:禁用,1:启用',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=36 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `perm_user`
--

LOCK TABLES `perm_user` WRITE;
/*!40000 ALTER TABLE `perm_user` DISABLE KEYS */;
INSERT INTO `perm_user` VALUES (19,'xxx2','xxxx',0,18,'ffffffff','fffffff','aaaa234','aaaa234','xxx','xxx','bff02e400c92372ce6d60596de5d8dd5',0,0,'2023-01-14 13:37:42','2023-02-02 23:03:31'),(20,'管理员','管理员',1,19,'2023-02-06','/upload/avatar/3a060ce93613cf2bb2cd961ddd33d0ca.gif','18312465088','xx23x@163.com','本人性格热情开朗，待人友好，为人诚实谦虚。工作勤奋，认真负责，能吃苦耐劳，尽职尽责，有耐心。具有亲和力，平易近人，善于与人沟通。','xxx','bff02e400c92372ce6d60596de5d8dd5',1,1,'2023-01-15 17:25:17','2023-02-26 00:00:20'),(21,'用户1','用户1',1,1,'2023-02-14','/upload/avatar/0ba837dd16e6895037a2a3f46c53098f.gif','18312465168','xxx@163.com','cs','saddas','bff02e400c92372ce6d60596de5d8dd5',2,1,'2023-02-03 00:07:53','2023-02-26 00:02:11'),(22,'','xxx',1,1,'xx','','18312465018','','','','bff02e400c92372ce6d60596de5d8dd5',0,1,'2023-02-03 00:19:43','2023-02-03 00:19:43'),(23,'dsas','dasd',0,2,'2023-02-15','','22222222222','dasdsad','asd','asd','bff02e400c92372ce6d60596de5d8dd5',0,1,'2023-02-03 21:49:45','2023-02-03 21:49:45'),(25,'33','aaaaaaaaaaa',2,4,'2023-02-01','/upload/avatar/合约.png','11111111113','33332','ad','asdasd','bff02e400c92372ce6d60596de5d8dd5',1,1,'2023-02-03 22:07:04','2023-02-18 16:28:21'),(31,'xxx2','demo',1,18,'xxx','xxx','183124650222','','xxx','xxx','bff02e400c92372ce6d60596de5d8dd5',0,1,'2023-02-05 23:15:30','2023-02-05 23:15:30'),(32,'xxxwqqwe','asdas',1,3,'2023-02-16','/upload/avatar/29b4daa0ab7ec88bec276e23c6db3a0f.gif','22222255555','','','','bff02e400c92372ce6d60596de5d8dd5',1,0,'2023-02-06 23:19:20','2023-03-11 09:56:03'),(33,'啊啊啊啊啊啊啊','asssss 232',1,3,'2023-02-16','','44444444445','','','','963ad6725d046bc76d4a72c6a7444f28',0,1,'2023-02-06 23:21:03','2023-02-06 23:21:03'),(34,'werwe','rwerwe',1,2,'2023-02-06','','22222222223','','','','7d4bff9d2194a9e3c53be923fd5ff21c',1,1,'2023-02-06 23:34:48','2023-02-22 20:26:22'),(35,'测试','测试',0,1,'2023-03-14','','18312465000','','','','bff02e400c92372ce6d60596de5d8dd5',0,1,'2023-03-11 09:53:39','2023-03-11 09:53:39');
/*!40000 ALTER TABLE `perm_user` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `perm_user_api_token`
--

DROP TABLE IF EXISTS `perm_user_api_token`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `perm_user_api_token` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `user_id` int(20) NOT NULL COMMENT '用户ID',
  `token` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '令牌',
  `passphrase` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '口令',
  `permission` varchar(20) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '权限:GET,POST,PUT,DELETE',
  `note` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '备注',
  `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用,0:禁用,1:启用',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户API接口Token令牌表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `perm_user_api_token`
--

LOCK TABLES `perm_user_api_token` WRITE;
/*!40000 ALTER TABLE `perm_user_api_token` DISABLE KEYS */;
INSERT INTO `perm_user_api_token` VALUES (4,32,'bf07b991433d2d835472f44078b356bc','手打','POST','',1,'2023-03-09 22:39:04','2023-03-11 00:13:37'),(5,20,'96f4705255f8a21e614f2a17cf76a27e','xxx','GET;POST;DELETE;PUT','xx',1,'2023-03-09 22:51:11','2023-03-11 00:07:25');
/*!40000 ALTER TABLE `perm_user_api_token` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `perm_user_role_rel`
--

DROP TABLE IF EXISTS `perm_user_role_rel`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `perm_user_role_rel` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `user_id` int(10) NOT NULL COMMENT '用户ID',
  `role_id` int(10) NOT NULL COMMENT '角色ID',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `user_role_rel_user_id` (`user_id`),
  KEY `user_role_rel_role_id` (`role_id`),
  CONSTRAINT `user_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `user_role_rel_user_id` FOREIGN KEY (`user_id`) REFERENCES `perm_user` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=23 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户角色关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `perm_user_role_rel`
--

LOCK TABLES `perm_user_role_rel` WRITE;
/*!40000 ALTER TABLE `perm_user_role_rel` DISABLE KEYS */;
INSERT INTO `perm_user_role_rel` VALUES (9,25,2,'2023-02-05 00:44:13','2023-02-05 00:44:13'),(10,25,1,'2023-02-05 00:44:13','2023-02-05 00:44:13'),(11,20,2,'2023-02-05 19:24:48','2023-02-05 19:24:48'),(12,20,1,'2023-02-05 19:24:48','2023-02-05 19:24:48'),(13,21,2,'2023-02-05 21:13:28','2023-02-05 21:13:28'),(16,34,13,'2023-02-19 23:45:39','2023-02-19 23:45:39'),(18,34,3,'2023-02-19 23:45:39','2023-02-19 23:45:39'),(19,34,2,'2023-02-19 23:45:48','2023-02-19 23:45:48'),(20,20,3,'2023-02-25 12:30:08','2023-02-25 12:30:08'),(21,35,3,'2023-03-11 09:53:39','2023-03-11 09:53:39'),(22,35,14,'2023-03-11 09:53:39','2023-03-11 09:53:39');
/*!40000 ALTER TABLE `perm_user_role_rel` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_config`
--

DROP TABLE IF EXISTS `sys_config`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_config` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '配置ID',
  `parent_id` int(11) DEFAULT NULL COMMENT '父节点ID',
  `name` varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '配置名称',
  `key` varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '配置参数(英文)',
  `value` text COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '配置参数值',
  `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
  `note` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '配置描述',
  `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用,0:禁用,1:启用',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `key` (`key`)
) ENGINE=InnoDB AUTO_INCREMENT=27 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='应用配置表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_config`
--

LOCK TABLES `sys_config` WRITE;
/*!40000 ALTER TABLE `sys_config` DISABLE KEYS */;
INSERT INTO `sys_config` VALUES (1,NULL,'系统配置','system','',1,'系统配置',1,'2023-02-14 22:17:20','2023-02-25 21:01:44'),(2,1,'应用名称','sys_app_name','后台管理系统',1,'',1,'2023-02-14 22:38:32','2023-02-19 23:37:26'),(3,1,'系统邮箱配置','sys_email','{}',1,'',1,'2023-02-19 23:35:23','2023-02-19 23:35:23'),(4,NULL,'网站配置','website','',1,'网站全局配置',1,'2023-02-25 20:32:41','2023-02-25 20:32:41'),(5,4,'网站全称','website_title','后台管理系统',1,'',1,'2023-02-25 20:33:49','2023-03-11 00:14:36'),(6,4,'网站简称','website_title_brief','基于Gin+Vue后台开发框架',1,'',1,'2023-02-25 20:34:48','2023-03-11 00:14:36'),(11,4,'网站LOGO','website_logo','/upload/images/2023-02-26/c3c9bfd0153679db77deec116aad3aa8.ico',1,'',1,'2023-02-25 20:50:14','2023-03-11 00:14:36'),(12,4,'网站SEO标题','website_seo_title','基于Go语言Gin、Vue、MySQL敏捷开发框架',1,'',1,'2023-02-25 20:51:16','2023-03-11 00:14:36'),(13,4,'网站SEO描述','website_seo_desc','基于Go语言Gin、Vue、MySQL等框架打造的组件式敏捷开发框架，简化开发，提升开发效率！！',1,'',1,'2023-02-25 20:51:38','2023-03-11 00:14:36'),(14,4,'网站版权信息','website_copyright','网站版权信息',1,'',1,'2023-02-25 20:52:14','2023-03-11 00:14:36'),(15,4,'网站关键词','website_keywords','go,Gin, vue,MySQL',1,'使用英文逗号\',\'',1,'2023-02-25 20:53:09','2023-03-11 00:14:36'),(16,4,'网站描述','website_description','基于Go语言Gin、Vue、MySQL等框架打造的组件式敏捷开发框架，简化开发，提升开发效率！！',1,'',1,'2023-02-25 20:53:42','2023-03-11 00:14:36'),(17,4,'公司地址','website_company_address','公司地址2',1,'',1,'2023-02-25 20:54:34','2023-03-11 00:14:36'),(18,4,'网站电话','website_phone','网站电话',1,'',1,'2023-02-25 20:54:57','2023-03-11 00:14:36'),(19,4,'网站邮箱','website_email','silent_rains@163.com',1,'',1,'2023-02-25 20:55:12','2023-03-11 00:14:36'),(20,4,'网站QQ','website_qq','2367221387',1,'',1,'2023-02-25 20:56:11','2023-03-11 00:14:36'),(21,4,'网站备案号','website_filing_number','网站备案号',1,'',1,'2023-02-25 20:58:28','2023-03-11 00:14:36'),(22,4,'网站宣传片','website_propaganda','[{\"name\":\"合约.png\",\"url\":\"/upload/images/2023-02-26/69aa018dd7a362e0db00cdd51bc3e7d0.png\"},{\"name\":\"f778738c-e4f8-4870-b634-56703b4acafe.gif\",\"url\":\"/upload/images/2023-02-26/46ba34c2b7de1729b5f7ba9b80ab6794.gif\"}]',1,'',1,'2023-02-25 21:06:14','2023-03-11 00:14:36'),(23,4,'网站标签','website_tags','test',1,'',1,'2023-02-25 21:06:41','2023-03-11 00:14:36'),(24,4,'网站开发者','website_anthor','silent-rain,silent_rains@163.com',1,'',1,'2023-03-08 23:26:12','2023-03-11 00:14:36'),(25,NULL,'API鉴权设置','api_auth','',1,'API鉴权设置',1,'2023-03-10 00:24:08','2023-03-10 00:24:08'),(26,25,'可申请最大令牌数','api_auth_max_token_num','5',1,'',1,'2023-03-10 00:25:41','2023-03-10 00:25:41');
/*!40000 ALTER TABLE `sys_config` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_icon`
--

DROP TABLE IF EXISTS `sys_icon`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_icon` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '配置ID',
  `name` varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '配置名称',
  `value` varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '配置值',
  `category` tinyint(1) NOT NULL DEFAULT 0 COMMENT '图标类型,1:element,2:custom',
  `note` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '配置描述',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='ICON图标表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_icon`
--

LOCK TABLES `sys_icon` WRITE;
/*!40000 ALTER TABLE `sys_icon` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_icon` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user_login`
--

DROP TABLE IF EXISTS `sys_user_login`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_user_login` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `user_id` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '用户ID',
  `nickname` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '用户昵称',
  `remote_addr` varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '登录IP',
  `user_agent` varchar(256) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '用户代理',
  `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '登录状态,0:禁用,1:启用',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp() COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=32 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户登录表-用于登录';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user_login`
--

LOCK TABLES `sys_user_login` WRITE;
/*!40000 ALTER TABLE `sys_user_login` DISABLE KEYS */;
INSERT INTO `sys_user_login` VALUES (8,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-07 22:17:16','2023-03-07 22:17:16'),(9,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-07 22:19:15','2023-03-07 22:19:15'),(10,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-07 22:21:42','2023-03-07 22:21:42'),(11,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-07 22:33:35','2023-03-07 22:33:35'),(12,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-07 22:33:51','2023-03-07 22:33:51'),(13,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-07 22:35:09','2023-03-07 22:35:09'),(14,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-07 22:36:11','2023-03-07 22:36:11'),(15,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',0,'2023-03-07 22:36:54','2023-03-07 22:47:56'),(16,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',0,'2023-03-07 22:49:50','2023-03-07 22:58:46'),(17,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',0,'2023-03-07 22:59:23','2023-03-07 22:59:32'),(18,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-07 22:59:40','2023-03-07 22:59:40'),(19,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-08 22:52:36','2023-03-08 22:52:36'),(20,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-08 23:10:44','2023-03-08 23:10:44'),(21,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-08 23:39:20','2023-03-08 23:39:20'),(22,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-09 00:28:34','2023-03-09 00:28:34'),(23,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-09 21:52:12','2023-03-09 21:52:12'),(24,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-09 22:24:27','2023-03-09 22:24:27'),(25,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-09 22:33:41','2023-03-09 22:33:41'),(26,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',0,'2023-03-10 23:02:14','2023-03-11 00:14:46'),(27,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-11 00:15:02','2023-03-11 00:15:02'),(28,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-11 10:04:35','2023-03-11 10:04:35'),(29,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-11 10:04:53','2023-03-11 10:04:53'),(30,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-11 10:05:26','2023-03-11 10:05:26'),(31,'20','管理员','127.0.0.1','Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36',1,'2023-03-11 10:06:02','2023-03-11 10:06:02');
/*!40000 ALTER TABLE `sys_user_login` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-03-11 16:18:29
