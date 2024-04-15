//! 定时任务

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询定时任务日志列表
#[derive(Default, Deserialize)]
pub struct GetScheduleJobLogListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 任务ID
    pub job_id: Option<i32>,
    /// 任务名称
    pub job_name: Option<String>,
}

/// 添加定时任务日志
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AddScheduleJobLogReq {
    /// 任务ID
    pub job_id: i32,
    /// 任务名称
    pub job_name: String,
    /// 失败信息
    pub error: Option<String>,
    /// 耗时(单位：毫秒)
    pub cost: f64,
    /// 任务状态,0:失败,1:成功'
    /// Enum: [`crate::schedule::enums::ScheduleJobStatus`]
    pub status: i8,
}
