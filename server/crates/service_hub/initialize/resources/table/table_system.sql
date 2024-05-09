/*系统相关表*/
-- 图片验证码表
CREATE TABLE IF NOT EXISTS
    `t_sys_image_captcha` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `captcha_id` VARCHAR(40) UNIQUE NOT NULL COMMENT '验证码ID',
        `captcha` VARCHAR(10) NOT NULL COMMENT '验证码',
        `data` MEDIUMBLOB NOT NULL COMMENT '图片数据, Base64编码',
        `expire` INT(4) UNSIGNED NOT NULL DEFAULT 1 COMMENT '过期时间,秒',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0:失效,1:有效)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '图片图片验证码表';

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
        KEY `idx_dimension_id` (`dimension_id`),
        KEY `idx_dimension_code` (`dimension_code`),
        CONSTRAINT `fk_sys_dict_data_dimension_id` FOREIGN KEY (`dimension_id`) REFERENCES `t_sys_dict_dimension` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '字典数据表';

-- TODO 会员等级表
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