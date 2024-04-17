/* Open Api 相关的表 */
-- OpenApi接口表
CREATE TABLE
    t_open_api (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '接口ID',
        `pid` INT(20) NULL DEFAULT 0 COMMENT '父ID',
        `category` VARCHAR(20) NOT NULL COMMENT '类别,0:目录,1:接口',
        `name` VARCHAR(50) NOT NULL COMMENT '接口名称',
        `method` VARCHAR(50) NOT NULL COMMENT '请求类型',
        `path` VARCHAR(200) NOT NULL COMMENT '资源路径',
        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
        `note` VARCHAR(200) NULL DEFAULT '' COMMENT '备注',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态, 0:停用,1:正常',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'OpenApi接口表';

-- OpenApi接口与角色关联表
CREATE TABLE
    t_open_api_role_rel (
        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `api_id` INT(11) NOT NULL COMMENT '接口ID',
        `role_id` INT(11) NOT NULL COMMENT '角色ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`),
        CONSTRAINT `t_open_api_role_rel_api_id` FOREIGN KEY (`api_id`) REFERENCES `t_open_api` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT `t_open_api_role_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `t_perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'OpenApi接口与角色关联表';