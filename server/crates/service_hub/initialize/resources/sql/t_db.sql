/*创建数据库*/
CREATE DATABASE IF NOT EXISTS `actix_admin_web` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;

/*用户与身份管理相关表*/
-- 用户表
CREATE TABLE IF NOT EXISTS
    `t_user_base` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '用户ID',
        `username` VARCHAR(32) NOT NULL COMMENT '用户名称',
        `real_name` VARCHAR(32) NULL DEFAULT '' COMMENT '真实姓名',
        `gender` TINYINT(1) NULL DEFAULT 0 COMMENT '性别(0:男,1:女,2:保密)',
        `age` INT(11) NULL DEFAULT 0 COMMENT '年龄',
        `birthday` VARCHAR(20) NULL DEFAULT '' COMMENT '出生日期',
        `avatar` VARCHAR(200) NULL DEFAULT '' COMMENT '头像URL',
        `intro` VARCHAR(200) NULL DEFAULT '' COMMENT '介绍',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `password` VARCHAR(64) NOT NULL COMMENT '密码',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0:停用,1:正常)',
        `department_id` INT(11) UNSIGNED DEFAULT 0 COMMENT '所属部门ID',
        `position_id` INT(11) UNSIGNED DEFAULT 0 COMMENT '所属岗位ID',
        `rank_id` INT(11) UNSIGNED DEFAULT 0 COMMENT '所属职级ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_username` (`username`) USING BTREE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户';

-- 用户邮箱表
CREATE TABLE IF NOT EXISTS
    `t_user_email` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '邮箱ID',
        `user_id` INT(10) NOT NULL COMMENT '用户ID',
        `email` VARCHAR(50) NOT NULL COMMENT '邮箱',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_user_id` (`user_id`) USING BTREE,
        UNIQUE KEY `uk_email` (`email`) USING BTREE,
        CONSTRAINT `fk_email_user_id` FOREIGN KEY (`user_id`) REFERENCES `t_user_base` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户邮箱';

-- 用户手机号表
CREATE TABLE IF NOT EXISTS
    `t_user_phone` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '手机号ID',
        `user_id` INT(10) NOT NULL COMMENT '用户ID',
        `phone` VARCHAR(16) NOT NULL COMMENT '手机号码',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_user_id` (`user_id`) USING BTREE,
        UNIQUE KEY `uk_phone` (`phone`) USING BTREE,
        CONSTRAINT `fk_user_phone_user_id` FOREIGN KEY (`user_id`) REFERENCES `t_user_base` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户手机号';

-- 用户区块链钱包
CREATE TABLE IF NOT EXISTS
    `t_user_blockchain_wallet` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '手机号ID',
        `user_id` INT(10) NOT NULL COMMENT '用户ID',
        `wallet_address` VARCHAR(255) NOT NULL COMMENT '钱包地址',
        `mnemonic` VARCHAR(255) NULL DEFAULT '' COMMENT '助记词',
        `private_key` VARCHAR(255) NULL DEFAULT '' COMMENT '私钥',
        `chain_id` INT(10) NULL DEFAULT 0 COMMENT '区块链ID',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_user_id` (`user_id`) USING BTREE,
        UNIQUE KEY `uk_wallet_address` (`wallet_address`) USING BTREE,
        CONSTRAINT `fk_perm_user_blockchain_wallet_user_id` FOREIGN KEY (`user_id`) REFERENCES `t_user_base` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户区块链钱包';

-- 用户角色关系表
CREATE TABLE IF NOT EXISTS
    `t_user_role_rel` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `user_id` INT(10) NOT NULL COMMENT '用户ID',
        `role_id` INT(10) NOT NULL COMMENT '角色ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_user_id_role_id` (`user_id`, `role_id`),
        CONSTRAINT `fk_user_role_rel_user_id` FOREIGN KEY (`user_id`) REFERENCES `t_user_base` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT `fk_user_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `t_perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户角色关系表';

-- 用户地理位置 - 待定
CREATE TABLE IF NOT EXISTS
    `t_user_location` (
        `id` INT UNSIGNED AUTO_INCREMENT NOT NULL COMMENT '地理位置ID',
        `province` VARCHAR(100) NOT NULL COMMENT '省份',
        `city` VARCHAR(100) NOT NULL COMMENT '城市',
        `district` VARCHAR(100) NOT NULL COMMENT '区/县',
        `address` VARCHAR(255) DEFAULT NULL COMMENT '详细地址',
        `postal_code` VARCHAR(20) DEFAULT NULL COMMENT '邮政编码',
        `longitude` DECIMAL(11, 8) DEFAULT NULL COMMENT '经度',
        `latitude` DECIMAL(10, 8) DEFAULT NULL COMMENT '纬度',
        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`location_id`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = '地理位置表';

-- 用户登录日志表
CREATE TABLE IF NOT EXISTS
    `t_log_user_login` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `user_id` INT(11) NOT NULL COMMENT '用户ID',
        `username` VARCHAR(32) NOT NULL COMMENT '用户名称',
        `token` VARCHAR(250) NOT NULL COMMENT '登陆令牌',
        `remote_addr` VARCHAR(64) NULL DEFAULT '' COMMENT '登录IP',
        `user_agent` VARCHAR(256) NULL DEFAULT '' COMMENT '用户代理',
        `device` VARCHAR(20) NULL DEFAULT '' COMMENT '设备',
        `system` VARCHAR(20) NULL DEFAULT '' COMMENT '系统',
        `browser` VARCHAR(20) NULL DEFAULT '' COMMENT '浏览器',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '登录状态(0:失败,1:成功)',
        `is_disabled` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否被禁用(0: 有效, 1: 被禁用)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户登录日志表';

/*
-- user表触发器，更新其他表冗余字段

CREATE TRIGGER trigger_update_user
AFTER
UPDATE
ON `user_base` FOR EACH ROW BEGIN

IF NEW.nickname != OLD.nickname THEN 
-- 更新 perm_user_api_token.nickname 字段
UPDATE
perm_user_api_token
SET
nickname = NEW.nickname
WHERE
user_id = NEW.id;
END IF;
END;
 */
/*
权限相关的表
 */
-- 角色表
CREATE TABLE IF NOT EXISTS
    `t_perm_role` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '角色ID',
        `name` VARCHAR(20) UNIQUE NOT NULL COMMENT '角色名称',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0:停用,1:正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '角色表';

-- 菜单表
CREATE TABLE IF NOT EXISTS
    `t_perm_menu` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '菜单ID',
        `pid` INT(20) NULL DEFAULT 0 COMMENT '父菜单ID',
        `title` VARCHAR(20) NOT NULL COMMENT '菜单名称',
        `icon` VARCHAR(20) NULL DEFAULT '' COMMENT 'Icon图标',
        `el_icon` VARCHAR(20) NULL DEFAULT '' COMMENT 'Element-Ico图标',
        `menu_type` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '菜单类型(0:菜单,1:按钮)',
        `open_type` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '打开方式(0:组件,1:内链,2:外链)',
        `path` VARCHAR(500) NULL DEFAULT '' COMMENT '路由地址',
        `component` VARCHAR(500) NULL DEFAULT '' COMMENT '组件路径',
        `redirect` VARCHAR(200) NULL DEFAULT '' COMMENT '路由重定向',
        `link` VARCHAR(200) NULL DEFAULT '' COMMENT '链接地址:站内链地址/站外链地址',
        `link_target` VARCHAR(20) NULL DEFAULT '_blank' COMMENT '链接跳转方式,_blank/_self',
        `hidden` TINYINT(1) NULL DEFAULT 1 COMMENT '是否隐藏(0:显示,1:隐藏)',
        `root_always_show` TINYINT(1) NULL DEFAULT 1 COMMENT '是否始终显示根菜单(0:隐藏,1:显示)',
        `permission` VARCHAR(200) NULL DEFAULT '' COMMENT '权限标识',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0:停用,1:正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '菜单表';

-- 菜单角色关系表
CREATE TABLE IF NOT EXISTS
    `t_perm_menu_role_rel` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `menu_id` INT(10) NOT NULL COMMENT '菜单ID',
        `role_id` INT(10) NOT NULL COMMENT '角色ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_menu_id_role_id` (`menu_id`, `role_id`),
        CONSTRAINT `fk_perm_menu_role_rel_menu_id` FOREIGN KEY (`menu_id`) REFERENCES `t_perm_menu` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT `fk_perm_menu_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `t_perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '菜单角色关系表';

-- 令牌表
CREATE TABLE IF NOT EXISTS
    `t_perm_token` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '令牌ID',
        `user_id` INT(20) NOT NULL COMMENT '用户ID',
        `token` VARCHAR(50) UNIQUE NOT NULL COMMENT '令牌',
        `passphrase` VARCHAR(20) NOT NULL COMMENT '口令',
        `permission` VARCHAR(20) NOT NULL COMMENT '权限范围:GET,POST,PUT,DELETE',
        `expire` DATETIME NOT NULL COMMENT '授权到期时间',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态, 0:停用,1:正常',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '令牌表';

-- 令牌角色关系表
CREATE TABLE IF NOT EXISTS
    t_perm_token_role_rel (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `token_id` INT(11) NOT NULL COMMENT '令牌ID',
        `role_id` INT(11) NOT NULL COMMENT '角色ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_token_id_role_id` (`token_id`, `role_id`),
        CONSTRAINT `fk_perm_token_role_rel_token_id` FOREIGN KEY (`token_id`) REFERENCES `t_perm_token` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT `fk_perm_token_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `t_perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '令牌角色关系表';

-- OpenApi接口表
CREATE TABLE IF NOT EXISTS
    t_perm_openapi (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '接口ID',
        `pid` INT(20) NULL DEFAULT 0 COMMENT '父ID',
        `category` TINYINT(1) NOT NULL COMMENT '类别,0:目录,1:接口',
        `name` VARCHAR(50) NOT NULL COMMENT '接口名称',
        `method` VARCHAR(50) NOT NULL COMMENT '请求类型',
        `path` VARCHAR(200) NOT NULL COMMENT '资源路径',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态, 0:停用,1:正常',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'OpenApi接口表';

-- OpenApi接口角色关系表
CREATE TABLE IF NOT EXISTS
    t_perm_openapi_role_rel (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `openapi_id` INT(11) NOT NULL COMMENT '接口ID',
        `role_id` INT(11) NOT NULL COMMENT '角色ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_openapi_id_role_id` (`openapi_id`, `role_id`),
        CONSTRAINT `fk_openapi_role_rel_openapi_id` FOREIGN KEY (`openapi_id`) REFERENCES `t_perm_openapi` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT `fk_openapi_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `t_perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'OpenApi接口角色关系表';

/*组织相关表*/
-- 部门表
CREATE TABLE IF NOT EXISTS
    `t_org_department` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '部门ID',
        `pid` BIGINT DEFAULT NULL DEFAULT 0 COMMENT '上级部门ID',
        `pids` VARCHAR(200) DEFAULT NULL DEFAULT '' COMMENT '所有上级部门ID, 用逗号分开',
        `name` VARCHAR(20) UNIQUE NOT NULL COMMENT '部门名称',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '部门描述',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0:停用,1:正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '部门表';

-- 部门角色关系表
CREATE TABLE IF NOT EXISTS
    `t_org_department_role_rel` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `department_id` INT(10) NOT NULL COMMENT '部门ID',
        `role_id` INT(10) NOT NULL COMMENT '角色ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_department_id_role_id` (`department_id`, `role_id`),
        CONSTRAINT `fk_org_department_role_rel_department_id` FOREIGN KEY (`department_id`) REFERENCES `t_org_department` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT `fk_org_department_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `t_perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '部门角色关系表';

-- 岗位表
CREATE TABLE IF NOT EXISTS
    `t_org_position` (
        `id` INT UNSIGNED AUTO_INCREMENT NOT NULL COMMENT '岗位ID',
        `name` VARCHAR(100) NOT NULL COMMENT '岗位名称',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '部门描述',
        `department_id` INT UNSIGNED DEFAULT NULL COMMENT '所属部门ID',
        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_name` (`name`),
        CONSTRAINT `fk_org_position_department_id` FOREIGN KEY (`id`) REFERENCES `t_org_department` (`id`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = '岗位表';

-- 职级表
CREATE TABLE IF NOT EXISTS
    `t_org_rank` (
        `id` INT UNSIGNED AUTO_INCREMENT NOT NULL COMMENT '职级ID',
        `name` VARCHAR(100) NOT NULL COMMENT '职级名称',
        `level` INT UNSIGNED NOT NULL COMMENT '职级等级',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '部门描述',
        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = '职级表';

/*
系统相关表
 */
-- 图片验证码表
CREATE TABLE IF NOT EXISTS
    `t_sys_image_captcha` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `captcha_id` VARCHAR(40) NOT NULL UNIQUE COMMENT '验证码ID',
        `captcha` VARCHAR(10) NOT NULL COMMENT '验证码',
        `base_img` MEDIUMBLOB NOT NULL COMMENT '图片数据, Base64编码',
        `expire` INT(4) UNSIGNED NOT NULL DEFAULT 1 COMMENT '过期时间,秒',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0:无效,1:有效)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '图片验证码表';

-- 配置表
CREATE TABLE IF NOT EXISTS
    `t_sys_config` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '配置ID',
        `pid` INT(11) DEFAULT 0 COMMENT '父节点ID',
        `name` VARCHAR(64) NOT NULL COMMENT '配置名称',
        `code` VARCHAR(64) UNIQUE NOT NULL COMMENT '配置编码(英文)',
        `value` TEXT NULL COMMENT '配置值',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) DEFAULT '' COMMENT '配置描述',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0:停用,1:正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '配置表';

-- 图片资源表
CREATE TABLE IF NOT EXISTS
    `t_sys_image_resource` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '图片ID',
        `name` VARCHAR(32) NOT NULL COMMENT '图片名称',
        `hash` VARCHAR(32) UNIQUE NOT NULL COMMENT '图片HASH值',
        `data` MEDIUMBLOB NOT NULL COMMENT '图片数据, Base64编码',
        `extension` VARCHAR(10) NOT NULL COMMENT '图片文件扩展名, 如svg, png',
        `size` INT(10) NOT NULL COMMENT '图片文件大小，单位为字节',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '图片资源表';

-- 字典维度表
CREATE TABLE IF NOT EXISTS
    `t_sys_dict_dimension` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '字典维度ID',
        `name` VARCHAR(64) UNIQUE NOT NULL COMMENT '字典维度名称',
        `code` VARCHAR(64) UNIQUE NOT NULL COMMENT '字典维度编码',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0: 停用, 1: 正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '字典维度表';

-- 字典数据表
CREATE TABLE IF NOT EXISTS
    `t_sys_dict_data` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '字典项ID',
        `dimension_id` INT(11) NOT NULL COMMENT '字典维度ID',
        `dimension_code` VARCHAR(64) NOT NULL COMMENT '字典维度编码',
        `lable` VARCHAR(64) NOT NULL COMMENT '字典项标签',
        `value` TEXT NOT NULL COMMENT '字典项值',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0: 停用, 1: 正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        CONSTRAINT `fk_sys_dict_data_dimension_id` FOREIGN KEY (`dimension_id`) REFERENCES `t_sys_dict_dimension` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '字典数据表';

-- 会员等级表
-- CONSTRAINT `fk_perm_user_member_level_id` FOREIGN KEY (`member_level_id`) REFERENCES `t_sys_member_level` (`id`) ON DELETE CASCADE
CREATE TABLE
    `t_sys_member_level` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '会员等级ID',
        `name` VARCHAR(660) NOT NULL COMMENT '会员等级名称',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '会员等级表';

/*
任务调度相关
 */
-- 任务调度
CREATE TABLE IF NOT EXISTS
    `t_schedule_job` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `name` VARCHAR(200) NOT NULL COMMENT '任务名称',
        `source` TINYINT(1) NOT NULL COMMENT '任务来源(0:用户定义,1:系统内部)',
        `job_type` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务类型(0:定时任务,1:即时任务)',
        `sys_code` VARCHAR(200) NOT NULL COMMENT '系统任务编码',
        `expression` VARCHAR(100) DEFAULT '' COMMENT 'cron表达式',
        `interval` INT(11) DEFAULT 0 COMMENT '间隔时间,秒',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '任务状态(0:下线,1:上线)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`) USING BTREE,
        UNIQUE KEY `uk_name` (`name`) USING BTREE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '任务调度';

-- 任务调度状态日志
CREATE TABLE IF NOT EXISTS
    `t_schedule_job_status_log` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '状态日志ID',
        `job_id` INT(11) NOT NULL COMMENT '任务ID',
        `uuid` VARCHAR(50) NOT NULL COMMENT '任务调度ID',
        `error` TEXT COMMENT '失败信息',
        `cost` INT(20) UNSIGNED NOT NULL COMMENT '耗时,毫秒',
        `status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务状态(0:开始,1:完成,2:停止,3:移除)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP() COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`) USING BTREE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '任务调度状态日志';

-- 任务调度事件日志
CREATE TABLE IF NOT EXISTS
    `t_schedule_job_event_log` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '事件日志ID',
        `job_id` INT(11) NOT NULL COMMENT '任务ID',
        `uuid` VARCHAR(50) NOT NULL COMMENT '任务调度ID',
        `status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务状态(0:开始,1:完成,2:停止,3:移除)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP() COMMENT '创建时间',
        PRIMARY KEY (`id`) USING BTREE,
        KEY `idx_job_id` (`job_id`) USING BTREE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '任务调度事件日志';

/*
日志相关表
 */
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
        `line` INT(10) NULL DEFAULT 0 COMMENT '报错行数',
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

-- WEB日志表
CREATE TABLE IF NOT EXISTS
    `t_log_web` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `user_id` INT(11) NULL DEFAULT 0 COMMENT '用户ID',
        `username` VARCHAR(32) NULL DEFAULT '' COMMENT '用户名称',
        `request_id` VARCHAR(32) NULL DEFAULT '' COMMENT '请求ID',
        `os_type` TINYINT(2) NOT NULL DEFAULT 0 COMMENT '终端类型(0: 未知,1: 安卓,2 :ios,3 :web)',
        `error_type` TINYINT(2) NOT NULL COMMENT '错误类型: 1:接口报错,2:代码报错',
        `level` VARCHAR(10) NOT NULL COMMENT '日志级别',
        `caller_line` VARCHAR(100) NOT NULL COMMENT '日发生位置',
        `url` VARCHAR(500) NOT NULL COMMENT '错误页面',
        `msg` TEXT NULL COMMENT '日志消息',
        `stack` TEXT NULL COMMENT '堆栈信息',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'WEB日志表';