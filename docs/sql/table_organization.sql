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

-- TODO 岗位表
CREATE TABLE IF NOT EXISTS
    `t_org_position` (
        `id` INT UNSIGNED AUTO_INCREMENT NOT NULL COMMENT '岗位ID',
        `name` VARCHAR(100) NOT NULL COMMENT '岗位名称',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '岗位描述',
        `department_id` INT UNSIGNED DEFAULT NULL COMMENT '所属部门ID',
        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`),
        UNIQUE KEY `uk_name` (`name`),
        CONSTRAINT `fk_org_position_department_id` FOREIGN KEY (`id`) REFERENCES `t_org_department` (`id`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = '岗位表';

-- TODO 职级表
CREATE TABLE IF NOT EXISTS
    `t_org_rank` (
        `id` INT UNSIGNED AUTO_INCREMENT NOT NULL COMMENT '职级ID',
        `name` VARCHAR(100) NOT NULL COMMENT '职级名称',
        `level` INT UNSIGNED NOT NULL COMMENT '职级等级',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '职级描述',
        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = '职级表';