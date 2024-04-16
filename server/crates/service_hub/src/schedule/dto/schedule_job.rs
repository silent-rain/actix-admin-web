//! 定时任务管理

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询定时任务列表
#[derive(Default, Deserialize, Validate)]
pub struct GetScheduleJobReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 任务名称
    pub name: Option<String>,
    /// 任务类型
    pub job_type: Option<i8>,
    /// 任务状态
    pub status: Option<i8>,
}

/// 添加定时任务
#[derive(Serialize, Deserialize, Validate)]
pub struct AddcheduleJobReq {
    /// 任务名称
    pub name: String,
    /// 任务来源,0:系统内部,1:用户定义
    /// Enum: [`crate::schedule::enums::ScheduleJobSource`]
    pub source: i8,
    /// 任务类型,0:定时任务,1:即时任务
    /// Enum: [`crate::schedule::enums::ScheduleJobType`]
    pub job_type: i8,
    /// cron表达式
    pub expression: Option<String>,
    /// 间隔时间,秒
    pub interval: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    /// Enum: [`crate::schedule::enums::ScheduleJobStatus`]
    pub status: i8,
}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdatecheduleJobReq {
    /// 任务名称
    pub name: String,
    /// 任务来源,0:系统内部,1:用户定义
    /// Enum: [`crate::schedule::enums::ScheduleJobSource`]
    pub source: i8,
    /// 任务类型,0:定时任务,1:即时任务
    /// Enum: [`crate::schedule::enums::ScheduleJobType`]
    pub job_type: i8,
    /// cron表达式
    pub expression: Option<String>,
    /// 间隔时间,秒
    pub interval: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    /// Enum: [`crate::schedule::enums::ScheduleJobStatus`]
    pub status: i8,
}

/// 更新数据状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdatecheduleJobStatusReq {
    /// 状态,0:停用,1:正常
    /// Enum: [`crate::schedule::enums::ScheduleJobStatus`]
    pub status: i8,
}
