/*
 日志相关表
 */
-- 网络请求日志表
CREATE TABLE log_http (
    `id` INT AUTO_INCREMENT COMMENT '自增ID',
    `user_id` INT NULL COMMENT '请求用户ID',
    `nickname` VARCHAR(32) NULL COMMENT '昵称',
    `trace_id` VARCHAR(32) NULL COMMENT '上游请求traceId',
    `error_code` INT(10) NOT NULL COMMENT '请求状态码',
    `method` VARCHAR(10) NOT NULL COMMENT '请求方法',
    `path` VARCHAR(500) NOT NULL COMMENT '请求地址路径',
    `query` VARCHAR(500) NULL COMMENT '请求参数',
    `body` LONGTEXT NULL COMMENT '请求体/响应体',
    `remote_addr` VARCHAR(64) NOT NULL COMMENT '请求IP',
    `user_agent` VARCHAR(256) NOT NULL COMMENT '用户代理',
    `cost` INT(20) NOT NULL COMMENT '耗时,纳秒',
    `htpp_type` VARCHAR(64) NOT NULL COMMENT '请求类型:REQ/RSP',
    `note` VARCHAR(255) NULL COMMENT '备注',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '网络请求日志';



-- 系统日志表
CREATE TABLE `log_system` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `user_id` int(20) DEFAULT NULL COMMENT '请求用户ID',
  `nickname` varchar(32) DEFAULT NULL COMMENT '昵称',
  `name` varchar(50) NOT NULL COMMENT '日志记录器名称',
  `parent_span_id` int(20) unsigned DEFAULT NULL COMMENT 'Parent Span Id',
  `span_id` int(20) unsigned DEFAULT NULL COMMENT 'Span Id',
  `module_path` varchar(100) DEFAULT NULL COMMENT '模块路径',
  `target` varchar(100) NOT NULL COMMENT '描述发生此元数据所描述的跨度或事件的系统部分',
  `file` varchar(500) DEFAULT NULL COMMENT '文件',
  `line` int(10) unsigned DEFAULT NULL COMMENT '报错行数',
  `level` varchar(10) NOT NULL COMMENT '日志级别',
  `kind` varchar(10) NOT NULL COMMENT '事件类型',
  `is_event` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否为事件',
  `is_span` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否为 span',
  `fields` varchar(500) DEFAULT NULL COMMENT '日志字段名称列表',
  `field_data` text DEFAULT NULL COMMENT 'fields 日志数据集',
  `message` text DEFAULT NULL COMMENT '日志信息',
  `code` int(10) DEFAULT NULL COMMENT '业务误码',
  `code_msg` varchar(500) DEFAULT NULL COMMENT '业务误码信息',
  `stack` text DEFAULT NULL COMMENT '堆栈信息',
  `note` varchar(255) DEFAULT NULL COMMENT '备注',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1485 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='系统日志';

-- WEB日志表
CREATE TABLE log_web (
    `id` INT AUTO_INCREMENT COMMENT '自增ID',
    `user_id` INT NULL COMMENT '请求用户ID',
    `nickname` VARCHAR(32) NULL COMMENT '昵称',
    `trace_id` VARCHAR(32) NULL COMMENT '请求traceId',
    `os_type` TINYINT(2) NOT NULL COMMENT '终端类型: 0: 未知,1: 安卓,2 :ios,3 :web',
    `error_type` TINYINT(2) NOT NULL COMMENT '错误类型: 1:接口报错,2:代码报错',
    `level` VARCHAR(10) NOT NULL COMMENT '日志级别',
    `caller_line` VARCHAR(100) NOT NULL COMMENT '日发生位置',
    `url` VARCHAR(500) NOT NULL COMMENT '错误页面',
    `msg` TEXT NULL COMMENT '日志消息',
    `stack` TEXT NULL COMMENT '堆栈信息',
    `note` VARCHAR(255) NULL COMMENT '备注',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'WEB日志表';