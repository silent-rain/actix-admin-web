//! 枚举

/// 定时任务状态
#[derive(Debug, Clone, PartialEq)]
#[repr(i8)]
pub enum ScheduleJobStatus {
    /// 下线
    Offline = 0,
    /// 上线
    Online = 1,
}

/// 定时任务类型
#[derive(Debug, Clone, PartialEq)]
#[repr(i8)]
pub enum ScheduleJobType {
    /// 定时任务
    Timer = 0,
    /// 即时任务
    Interval = 1,
}

/// 定时任务来源
#[derive(Debug, Clone, PartialEq)]
#[repr(i8)]
pub enum ScheduleJobSource {
    /// 系统内部
    System = 0,
    /// 用户定义
    User = 1,
}

/// 定时任务运行状态
#[derive(Debug, Clone, PartialEq)]
#[repr(i8)]
pub enum ScheduleJobLogStatus {
    /// 待执行
    Pending = 0,
    /// 运行中
    Running = 1,
    /// 成功
    Success = 2,
    /// 失败
    Failed = 3,
    /// 移除
    Removed = 4,
}
