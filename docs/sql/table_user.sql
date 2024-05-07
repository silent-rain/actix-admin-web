/*
用户与身份管理相关表
 */
-- 用户表
CREATE TABLE IF NOT EXISTS
    `t_perm_user` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '用户ID',
        `username` VARCHAR(32) NOT NULL COMMENT '用户名称',
        `real_name` VARCHAR(32) NULL DEFAULT '' COMMENT '真实姓名',
        `gender` TINYINT(1) NULL DEFAULT 0 COMMENT '性别, 0:男,1:女,2:保密',
        `age` INT(11) NULL DEFAULT 0 COMMENT '年龄',
        `birthday` VARCHAR(20) NULL DEFAULT '' COMMENT '出生日期',
        `avatar` VARCHAR(200) NULL DEFAULT '' COMMENT '头像URL',
        `intro` VARCHAR(200) NULL DEFAULT '' COMMENT '介绍',
        `note` VARCHAR(200) NULL DEFAULT '' COMMENT '备注',
        `password` VARCHAR(64) NOT NULL COMMENT '密码',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态,0:停用,1:正常',
        `dept_id` BIGINT DEFAULT NULL DEFAULT 0 COMMENT '部门ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_username` (`username`) USING BTREE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户';

-- 用户邮箱表
CREATE TABLE IF NOT EXISTS
    `t_perm_user_email` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '邮箱ID',
        `user_id` INT(10) NOT NULL COMMENT '用户ID',
        `email` VARCHAR(50) NOT NULL COMMENT '邮箱',
        `note` VARCHAR(200) NULL DEFAULT '' COMMENT '备注',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_user_id` (`user_id`) USING BTREE,
        UNIQUE KEY `uk_email` (`email`) USING BTREE,
        CONSTRAINT `fk_perm_user_email_user_id` FOREIGN KEY (`user_id`) REFERENCES `t_perm_user` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户邮箱';

-- 用户手机号表
CREATE TABLE IF NOT EXISTS
    `t_perm_user_phone` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '手机号ID',
        `user_id` INT(10) NOT NULL COMMENT '用户ID',
        `phone` VARCHAR(16) NOT NULL COMMENT '手机号码',
        `note` VARCHAR(200) NULL DEFAULT '' COMMENT '备注',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_user_id` (`user_id`) USING BTREE,
        UNIQUE KEY `uk_phone` (`phone`) USING BTREE,
        CONSTRAINT `fk_perm_user_phone_user_id` FOREIGN KEY (`user_id`) REFERENCES `t_perm_user` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
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
        `note` VARCHAR(200) NULL DEFAULT '' COMMENT '备注',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_user_id` (`user_id`) USING BTREE,
        UNIQUE KEY `uk_wallet_address` (`wallet_address`) USING BTREE,
        CONSTRAINT `fk_perm_user_blockchain_wallet_user_id` FOREIGN KEY (`user_id`) REFERENCES `t_perm_user` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户区块链钱包';

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
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '登录状态,0:失败,1:成功',
        `disabled` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '禁用状态,0:未禁用,1:禁用',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户登录日志表';

-- 用户地理位置 - 待定
CREATE TABLE IF NOT EXISTS
    `t_perm_user_location` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '位置ID',
        `user_id` VARCHAR(10) NOT NULL COMMENT '用户ID',
        `province_code` VARCHAR(10) NULL DEFAULT '' COMMENT '省',
        `city_code` VARCHAR(10) NULL DEFAULT '' COMMENT '市',
        `district_code` VARCHAR(10) NULL DEFAULT '' COMMENT '区',
        `address` VARCHAR(200) NULL DEFAULT '' COMMENT '居住地址',
        `ad_code` VARCHAR(10) NULL DEFAULT '' COMMENT '地理编号',
        `lng` VARCHAR(20) NULL DEFAULT '' COMMENT '城市坐标中心点经度 （ * 1e6 ） ： 如果是中国 ， 此值是 1e7',
        `lat` VARCHAR(20) NULL DEFAULT '' COMMENT '城市坐标中心点纬度 （ * 1e6 ）',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        CONSTRAINT `fk_perm_user_location_user_id` FOREIGN KEY (`user_id`) REFERENCES `perm_user` (`id`) ON DELETE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户地理位置';

/*
-- user表触发器，更新其他表冗余字段

CREATE TRIGGER trigger_update_user
AFTER
UPDATE
ON `perm_user` FOR EACH ROW BEGIN

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