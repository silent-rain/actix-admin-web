/* API授权相关的表 */
-- 接口权限表
CREATE TABLE api_auth (
    `id` INT AUTO_INCREMENT COMMENT '自增ID',
    `pid` INT(20) NULL COMMENT '父ID',
    `name` VARCHAR(50) NOT NULL COMMENT '接口名称',
    `method` VARCHAR(50) NOT NULL COMMENT '请求类型',
    `uri` VARCHAR(50) NOT NULL COMMENT 'URI资源',
    `note` VARCHAR(200) NULL COMMENT '备注',
    `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态, 0:停用,1:正常',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '接口权限表';

-- 角色与接口关联表
CREATE TABLE api_role_http_rel (
    `id` INT AUTO_INCREMENT COMMENT '自增ID',
    `role_id` INT(11) NOT NULL COMMENT '角色ID',
    `api_id` INT(11) NOT NULL COMMENT '接口ID',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    CONSTRAINT `api_role_http_rel_role_id` FOREIGN KEY (`role_id`) REFERENCES `perm_role` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `api_role_http_rel_api_id` FOREIGN KEY (`api_id`) REFERENCES `api_http` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '角色与Http协议接口关联表';
