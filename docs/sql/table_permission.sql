-- 创建数据库 
CREATE DATABASE `actix_admin_web` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;

-- 角色表
CREATE TABLE
    perm_role (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '角色ID',
        `name` VARCHAR(20) UNIQUE NOT NULL COMMENT '角色名称',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `note` VARCHAR(200) NULL DEFAULT '' COMMENT '备注',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态,0:停用,1:正常',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '角色表';

/*
权限相关的表
 */
-- 用户表
CREATE TABLE
    perm_user (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '用户ID',
        `username` VARCHAR(32) NOT NULL COMMENT '用户名称',
        `real_name` VARCHAR(32) NULL DEFAULT '' COMMENT '真实姓名',
        `gender` TINYINT(1) NULL DEFAULT 0 COMMENT '性别, 0:男,1:女,2:保密',
        `age` INT(11) NULL DEFAULT 0 COMMENT '年龄',
        `birthday` VARCHAR(20) NULL DEFAULT '' COMMENT '出生日期',
        `avatar` VARCHAR(200) NULL DEFAULT '' COMMENT '头像URL',
        `phone` VARCHAR(20) NULL DEFAULT '' COMMENT '手机号码',
        `email` VARCHAR(100) NULL DEFAULT '' COMMENT '邮箱',
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

-- 用户角色关联表
CREATE TABLE
    perm_user_role_rel (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `user_id` INT(10) NOT NULL COMMENT '用户ID',
        `role_id` INT(10) NOT NULL COMMENT '角色ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uni_user_id_role_id` (`user_id`, `role_id`),
        CONSTRAINT `perm_user_role_rel_user_id` FOREIGN KEY (`user_id`) REFERENCES `perm_user` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT `perm_user_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户角色关联表';

-- 菜单表
CREATE TABLE
    perm_menu (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '菜单ID',
        `parent_id` INT(20) NULL DEFAULT 0 COMMENT '父菜单ID',
        `title` VARCHAR(20) NOT NULL COMMENT '菜单名称',
        `icon` VARCHAR(20) NULL DEFAULT '' COMMENT '菜单图标',
        `el_svg_icon` VARCHAR(20) NULL DEFAULT '' COMMENT 'Element菜单图标',
        `menu_type` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '菜单类型,0:菜单,1:按钮',
        `open_type` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '打开方式,0:组件,1:内链,2:外链',
        `path` VARCHAR(500) NULL DEFAULT '' COMMENT '路由地址',
        `name` VARCHAR(50) NULL DEFAULT '' COMMENT '路由别名',
        `component` VARCHAR(500) NULL DEFAULT '' COMMENT '组件路径',
        `redirect` VARCHAR(500) NULL DEFAULT '' COMMENT '路由重定向',
        `link` VARCHAR(500) NULL DEFAULT '' COMMENT '链接地址:内链地址/外链地址',
        `target` VARCHAR(500) NULL DEFAULT '' COMMENT '链接地址跳转方式, component/_blank/_self',
        `permission` VARCHAR(200) NULL DEFAULT '' COMMENT '权限标识',
        `hidden` TINYINT(1) NULL DEFAULT 1 COMMENT '是否隐藏,0:显示,1:隐藏',
        `always_show` TINYINT(1) NULL DEFAULT 1 COMMENT '始终显示根菜单,0:显示,1:隐藏',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `note` VARCHAR(200) NULL DEFAULT '' COMMENT '备注',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态,0:停用,1:启用',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '菜单表';

-- 菜单角色关联表
CREATE TABLE
    perm_menu_role_rel (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `role_id` INT(10) NOT NULL COMMENT '角色ID',
        `menu_id` INT(10) NOT NULL COMMENT '菜单ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`),
        CONSTRAINT `perm_menu_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT `perm_menu_role_rel_menu_id` FOREIGN KEY (`menu_id`) REFERENCES `perm_menu` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '菜单角色关联表';

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
-- 用户Token令牌表
CREATE TABLE
    perm_user_token (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '令牌ID',
        `user_id` INT(20) NOT NULL COMMENT '用户ID',
        `token` VARCHAR(50) NOT NULL COMMENT '令牌',
        `passphrase` VARCHAR(50) NOT NULL COMMENT '口令',
        -- `permission` VARCHAR(20) NOT NULL COMMENT '权限:GET,POST,PUT,DELETE',
        `expire` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '授权到期时间',
        `note` VARCHAR(200) NULL DEFAULT '' COMMENT '备注',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态,0:禁用,1:启用',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户Token令牌表, 一般openapi服务';

/*用户地理位置 - 待定
CREATE TABLE
_perm_user_location (
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
CONSTRAINT `perm_user_location_user_id` FOREIGN KEY (`user_id`) REFERENCES `perm_user` (`id`) ON DELETE CASCADE
) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户地理位置';
 */
/*用户头像表 - 待定, 可上传至服务器中
CREATE TABLE
_perm_user_avatar (
`id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '头像ID',
`user_id` VARCHAR(10) NOT NULL COMMENT '用户ID',
`data` LONGBLOB NOT NULL COMMENT '头像数据',
`hash` VARCHAR(50) NOT NULL COMMENT '头像hash值',
`created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '用户头像';
 */
/* 待定
- 岗位 职级
 */
-- 部门表
CREATE TABLE
    perm_dept (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '部门ID',
        `pid` BIGINT DEFAULT NULL DEFAULT 0 COMMENT '上级部门ID',
        `pids` VARCHAR(200) DEFAULT NULL DEFAULT '' COMMENT '所有上级部门ID, 用逗号分开',
        `name` VARCHAR(20) UNIQUE NOT NULL COMMENT '部门名称',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `note` VARCHAR(200) NULL DEFAULT '' COMMENT '备注',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态,0:停用,1:正常',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '部门表';

-- 部门角色关联表-数据权限
CREATE TABLE
    perm_dept_role_rel (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `role_id` INT(10) NOT NULL COMMENT '角色ID',
        `dept_id` INT(10) NOT NULL COMMENT '部门ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uni_user_id_role_id` (`role_id`, `dept_id`),
        CONSTRAINT `perm_dept_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT `perm_dept_role_rel_dept_id` FOREIGN KEY (`dept_id`) REFERENCES `perm_dept` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '部门角色关联表-数据权限';