/*
定时任务相关
 */
-- 定时任务
CREATE TABLE
  `t_schedule_job` (
    `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '定时任务ID',
    `name` VARCHAR(200) NOT NULL COMMENT '任务名称',
    `source` TINYINT(1) NOT NULL COMMENT '任务来源,0:系统内部,1:用户定义',
    `job_type` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务类型,0:定时任务,1:即时任务',
    `expression` VARCHAR(100) DEFAULT '' COMMENT 'cron表达式',
    `interval` INT(11) DEFAULT 0 COMMENT '间隔时间,秒',
    `note` VARCHAR(200) NULL DEFAULT '' COMMENT '备注',
    `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '任务状态,0:暂停,1:正常',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`) USING BTREE
  ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '定时任务';

-- 定时任务日志
CREATE TABLE
  `t_schedule_job_log` (
    `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '日志ID',
    `job_id` INT(11) NOT NULL COMMENT '任务ID',
    `job_name` VARCHAR(200) NOT NULL COMMENT '任务名称',
    `error` TEXT COMMENT '失败信息',
    `cost` DECIMAL(10, 2) NOT NULL COMMENT '耗时(单位：毫秒)',
    `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '任务状态,0:待运行,1:运行中,2:停止,3:成功,4:移除任务',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP() COMMENT '创建时间',
    PRIMARY KEY (`id`) USING BTREE,
    KEY `idx_job_id` (`job_id`) USING BTREE
  ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '定时任务日志';