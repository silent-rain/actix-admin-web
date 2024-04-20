//! 枚举

use serde_repr::{Deserialize_repr, Serialize_repr};

/// 定时任务状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ScheduleJobStatus {
    /// 暂停
    Paused = 0,
    /// 正常
    Normal = 1,
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
    /// 系统内部
    System = 0,
    /// 用户定义
    User = 1,
}

/// 定时任务运行状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ScheduleJobLogStatus {
    /// 失败
    Failed = 0,
    /// 成功
    Success = 1,
}
