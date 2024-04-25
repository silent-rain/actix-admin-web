//! 枚举

use serde_repr::{Deserialize_repr, Serialize_repr};

/// 定时任务状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ScheduleJobStatus {
    /// 下线
    Offline = 0,
    /// 上线
    Online = 1,
}

/// 定时任务类型
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ScheduleJobType {
    /// 定时任务
    Timer = 0,
    /// 即时任务
    Interval = 1,
}

/// 定时任务来源
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ScheduleJobSource {
    /// 用户定义
    User = 0,
    /// 系统内部
    System = 1,
}

/// 定时任务运行状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ScheduleJobLogStatus {
    /// 待执行
    Pending = 0,
    /// 运行中
    Running = 1,
    /// 暂停
    Paused = 2,
    /// 成功
    Success = 3,
    /// 失败
    Failed = 4,
    /// 移除
    Removed = 5,
}
