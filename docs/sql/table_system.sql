/*
系统相关表
 */
-- 验证码表
CREATE TABLE
    sys_captcha (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `captcha_id` VARCHAR(40) NOT NULL UNIQUE COMMENT '验证码ID',
        `captcha` VARCHAR(10) NOT NULL COMMENT '验证码',
        `base_img` LONGBLOB NOT NULL COMMENT 'Base64图片',
        `expire` INT(4) UNSIGNED NOT NULL DEFAULT 1 COMMENT '过期时间,秒',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态,0:无效,1:有效',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '验证码表';

-- 配置表
CREATE TABLE
    sys_config (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '配置ID',
        `pid` INT(11) DEFAULT NULL COMMENT '父节点ID',
        `name` VARCHAR(64) NOT NULL COMMENT '配置名称',
        `code` VARCHAR(64) NOT NULL UNIQUE COMMENT '配置编码(英文)',
        `value` TEXT COMMENT '配置值',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) DEFAULT NULL COMMENT '配置描述',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态,0:停用,1:正常',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '配置表';

-- ICON图标表
CREATE TABLE
    sys_icon (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '图标ID',
        `name` VARCHAR(32) NOT NULL UNIQUE COMMENT '图标名称',
        `base_img` LONGBLOB NOT NULL COMMENT 'Base64图片',
        `category` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '图标类型,1:element,2:custom',
        `note` VARCHAR(200) DEFAULT NULL COMMENT '备注',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'ICON图标表';

-- 字典维度表
CREATE TABLE
    sys_dict_dim (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '字典维度ID',
        `name` VARCHAR(64) NOT NULL UNIQUE COMMENT '字典维度名称',
        `code` VARCHAR(64) NOT NULL UNIQUE COMMENT '字典维度编码',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `note` VARCHAR(200) NULL COMMENT '备注',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态,0:停用,1:正常',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '字典维度表';

-- 字典数据表
CREATE TABLE
    sys_dict_data (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '字典项ID',
        `dim_id` INT(11) NOT NULL COMMENT '字典维度ID',
        `dim_code` VARCHAR(64) NOT NULL COMMENT '字典维度编码',
        `lable` VARCHAR(64) NOT NULL COMMENT '字典标签',
        `value` TEXT NOT NULL COMMENT '字典键值',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `note` VARCHAR(200) NULL COMMENT '备注',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态,0:停用,1:正常',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        CONSTRAINT `sys_dict_data_dim_id` FOREIGN KEY (`dim_id`) REFERENCES `sys_dict_dim` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '字典数据表';