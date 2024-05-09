/*权限相关的表*/
-- 菜单表
CREATE TABLE IF NOT EXISTS
    `t_perm_menu` (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '菜单ID',
        `pid` INT(20) NULL DEFAULT 0 COMMENT '父菜单ID',
        `title` VARCHAR(20) NOT NULL COMMENT '菜单名称',
        `icon_class` VARCHAR(20) NULL DEFAULT '' COMMENT 'Icon图标类',
        `menu_type` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '菜单类型(0:菜单,1:按钮)',
        `open_method` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '打开方式(0:组件,1:内链,2:外链)',
        `path` VARCHAR(255) NULL DEFAULT '' COMMENT '路由地址',
        `component_path` VARCHAR(255) NULL DEFAULT '' COMMENT '组件路径',
        `redirect_to` VARCHAR(255) NULL DEFAULT '' COMMENT '路由重定向',
        `link` VARCHAR(255) NULL DEFAULT '' COMMENT '链接地址:站内链地址/站外链地址',
        `link_target` VARCHAR(20) NULL DEFAULT '_blank' COMMENT '链接跳转方式,_blank/_self',
        `is_hidden` TINYINT(1) NULL DEFAULT 1 COMMENT '是否隐藏(0:显示,1:隐藏)',
        `is_always_show_root` TINYINT(1) NULL DEFAULT 1 COMMENT '是否始终显示根菜单(0:隐藏,1:显示)',
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
        CONSTRAINT `fk_perm_menu_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `t_user_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
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
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0:停用,1:正常)',
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
        CONSTRAINT `fk_perm_token_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `t_user_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
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
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0:停用,1:正常)',
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
        CONSTRAINT `fk_openapi_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `t_user_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'OpenApi接口角色关系表';