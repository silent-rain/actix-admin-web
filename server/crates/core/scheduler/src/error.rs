//! 定时任务错误类型
pub enum Error {
    /// 获取任务列表失败
    ScheduleJobListError(String),
    /// 获取Job任务失败
    JobSchedulerError(String),
    /// 更新数据库任务信息失败
    DbUpdateScheduleJobError(String),
    /// 未配置定时任务Expression
    NotExpressionError,
}
