//! 枚举

use serde::{Deserialize, Serialize};

/// 定时任务状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScheduleJobStatus {
    /// 暂停
    Paused = 0,
    /// 正常
    Normal = 1,
}

/// 定时任务类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScheduleJobType {
    /// 定时任务
    Timer = 0,
    /// 即时任务
    Interval = 1,
}

/// 定时任务运行状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScheduleJobLogStatus {
    /// 失败
    Failed = 0,
    /// 成功
    Success = 1,
}
