//! 任务调度状态日志管理

use entity::schedule_job_status_log;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询任务调度状态日志列表
#[derive(Default, Deserialize)]
pub struct GetScheduleJobStatusListLogReq {
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
    /// 任务状态
    pub status: Option<i8>,
}

/// 添加任务调度状态日志
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct AddScheduleJobStatusLogReq {
    /// 任务ID
    pub job_id: i32,
    /// 任务调度ID
    pub uuid: String,
}

/// 更新任务调度状态日志
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateScheduleJobStatusLogReq {
    /// 任务ID
    pub job_id: i32,
    /// 任务调度ID
    pub uuid: String,
    /// 失败信息
    pub error: Option<String>,
    /// 耗时,毫秒
    pub cost: u64,
    /// 任务状态,0:失败,1:成功
    pub status: schedule_job_status_log::enums::Status,
}

/// 更新数据状态
#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateScheduleJobStatusLogSatausReq {
    /// 任务状态,0:失败,1:成功
    pub status: schedule_job_status_log::enums::Status,
}
