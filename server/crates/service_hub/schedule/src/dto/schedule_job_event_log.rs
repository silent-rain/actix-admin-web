//! 任务调度事件日志管理

use entity::schedule_job_event_log;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询任务调度事件日志列表
#[derive(Default, Deserialize)]
pub struct GetScheduleJobEventLogListReq {
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
}

/// 添加任务调度事件日志
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct AddScheduleJobEventLogReq {
    /// 任务ID
    pub job_id: i32,
    /// 任务调度ID
    pub uuid: String,
    /// 任务状态,0:失败,1:成功'
    pub status: schedule_job_event_log::enums::Status,
}
