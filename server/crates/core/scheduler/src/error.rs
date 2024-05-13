//! 定时任务错误类型
use tokio_cron_scheduler::JobSchedulerError;

#[derive(Debug, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    #[error("Get Schedule Instance Error")]
    GetScheduleInstance,
    #[error("Init Schedule Instance Error, {0}")]
    InitScheduleInstance(String),
    #[error("Schedule Job List Error, {0}")]
    ScheduleJobListError(String),
    #[error("Job Scheduler Error, {0}")]
    JobSchedulerError(#[from] JobSchedulerError),
    #[error("Db Update Schedule Job Error, {0}")]
    DbUpdateScheduleJobError(String),
    #[error("Not Expression Error")]
    NotExpressionError,
    #[error("Not Interval Error")]
    NotIntervalError,
    #[error("任务来源错误")]
    ModelSourceError,
    #[error("解析 Uuid 失败, {0}")]
    ParseUuidError(String),
    #[error("为初始化Job")]
    NotInitJob,
}
