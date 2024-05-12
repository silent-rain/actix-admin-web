/*日志相关表*/
-- 用户登录日志表
CREATE TABLE IF NOT EXISTS
    `t_log_user_login` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `user_id` INT(11) NOT NULL COMMENT '用户ID',
        `username` VARCHAR(32) NOT NULL COMMENT '用户名称',
        `token` VARCHAR(300) NULL DEFAULT '' COMMENT '登陆令牌',
        `remote_addr` VARCHAR(64) NULL DEFAULT '' COMMENT '登录IP',
        `user_agent` VARCHAR(256) NULL DEFAULT '' COMMENT '用户代理',
        `device` VARCHAR(20) NULL DEFAULT '' COMMENT '设备',
        `system` VARCHAR(20) NULL DEFAULT '' COMMENT '系统',
        `browser` VARCHAR(20) NULL DEFAULT '' COMMENT '浏览器',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '登录状态(0:登陆成功,1:登陆失败,2: 已禁用)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户登录日志表';

-- API操作日志表
CREATE TABLE IF NOT EXISTS
    `t_log_api_operation` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `user_id` INT(11) NULL DEFAULT 0 COMMENT '用户ID',
        `username` VARCHAR(32) NULL DEFAULT '' COMMENT '用户名称',
        `request_id` VARCHAR(32) NULL DEFAULT '' COMMENT '请求ID',
        `status_code` INT(10) NOT NULL COMMENT '请求状态码',
        `method` VARCHAR(10) NOT NULL COMMENT '请求方法',
        `path` VARCHAR(500) NOT NULL COMMENT '请求地址路径',
        `query` VARCHAR(500) NULL DEFAULT '' COMMENT '请求参数',
        `body` TEXT NULL COMMENT '请求体/响应体',
        `remote_addr` VARCHAR(64) NULL DEFAULT '' COMMENT '请求IP',
        `user_agent` VARCHAR(256) NULL DEFAULT '' COMMENT '用户代理',
        `cost` INT(20) UNSIGNED NOT NULL COMMENT '耗时,毫秒',
        `http_type` VARCHAR(10) NOT NULL COMMENT '请求类型:REQ/RSP',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'API操作日志表';

-- 系统日志表
CREATE TABLE IF NOT EXISTS
    `t_log_system` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `user_id` INT(20) NULL DEFAULT 0 COMMENT '请求用户ID',
        `username` VARCHAR(32) NULL DEFAULT '' COMMENT '用户名称',
        `name` VARCHAR(50) NOT NULL COMMENT '日志记录器名称',
        `span_pid` INT(20) NULL DEFAULT 0 COMMENT 'Span Parent Id',
        `span_id` INT(20) NULL DEFAULT 0 COMMENT 'Span Id',
        `module_path` VARCHAR(100) NULL DEFAULT '' COMMENT '模块路径',
        `target` VARCHAR(100) NULL DEFAULT '' COMMENT '描述发生此元数据所描述的跨度或事件的系统部分',
        `file` VARCHAR(500) NULL DEFAULT '' COMMENT '文件',
        `line` INT(10) UNSIGNED NULL DEFAULT 0 COMMENT '报错行数',
        `level` VARCHAR(10) NOT NULL DEFAULT '' COMMENT '日志级别',
        `kind` VARCHAR(10) NOT NULL DEFAULT '' COMMENT '事件类型',
        `is_event` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否为事件',
        `is_span` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否为 span',
        `fields` VARCHAR(500) NULL DEFAULT '' COMMENT '日志字段名称列表',
        `field_data` TEXT NULL COMMENT 'fields 日志数据集',
        `message` TEXT NULL COMMENT '日志信息',
        `code` INT(10) NULL DEFAULT 0 COMMENT '业务误码',
        `code_msg` VARCHAR(500) NULL DEFAULT '' COMMENT '业务误码信息',
        `stack` TEXT NULL COMMENT '堆栈信息',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP() COMMENT '创建时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB AUTO_INCREMENT = 1485 DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = '系统日志';

-- TODO WEB日志表
CREATE TABLE IF NOT EXISTS
    `t_log_web` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '日志ID',
        `user_id` INT(11) NULL DEFAULT 0 COMMENT '用户ID',
        `username` VARCHAR(32) NULL DEFAULT '' COMMENT '用户名称',
        `request_id` VARCHAR(32) NULL DEFAULT '' COMMENT '请求ID',
        `os_type` TINYINT(1) NOT NULL COMMENT '终端类型(0:未知, 1:安卓, 2:IOS, 3:WEB)',
        `error_type` TINYINT(1) NOT NULL COMMENT '错误类型(1:接口报错, 2:代码报错)',
        `level` VARCHAR(10) NOT NULL COMMENT '日志级别',
        `caller_line` VARCHAR(100) NOT NULL COMMENT '日发生位置',
        `url` VARCHAR(500) NULL COMMENT '请求地址',
        `msg` TEXT NULL COMMENT '日志消息',
        `stack` TEXT NULL COMMENT '堆栈信息',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'WEB日志表';