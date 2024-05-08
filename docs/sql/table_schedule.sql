/*
任务调度相关
 */
-- 任务调度信息表
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
  ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '任务调度信息表';

-- 任务调度状态日志表
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
  ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '任务调度状态日志表';

-- 任务调度事件日志表
CREATE TABLE IF NOT EXISTS
  `t_schedule_job_event_log` (
    `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '事件日志ID',
    `job_id` INT(11) NOT NULL COMMENT '任务ID',
    `uuid` VARCHAR(50) NOT NULL COMMENT '任务调度ID',
    `status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务状态(0:开始,1:完成,2:停止,3:移除)',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP() COMMENT '创建时间',
    PRIMARY KEY (`id`) USING BTREE,
    KEY `idx_job_id` (`job_id`) USING BTREE
  ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '任务调度事件日志表';