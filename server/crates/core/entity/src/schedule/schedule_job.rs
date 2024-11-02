//! 任务调度作业表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 任务调度作业表
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_schedule_job")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 任务名称
    #[sea_orm(unique)]
    pub name: String,
    /// 任务来源(0:用户定义,1:系统内部)
    pub source: i8,
    /// 任务类型(0:定时任务,1:即时任务)
    pub job_type: i8,
    /// 系统任务编码
    pub sys_code: Option<String>,
    /// cron表达式
    pub expression: Option<String>,
    /// 间隔时间,秒
    pub interval: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 任务状态(0:下线,1:上线)
    pub status: i8,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// 枚举
pub mod enums {
    use serde_repr::{Deserialize_repr, Serialize_repr};

    /// 任务调度状态
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Status {
        /// 下线
        Offline = 0,
        /// 上线
        Online = 1,
    }

    /// 定时任务类型
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum JobType {
        /// 任务调度
        Timer = 0,
        /// 即时任务
        Interval = 1,
    }

    /// 定时任务来源
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Source {
        /// 用户定义
        User = 0,
        /// 系统内部
        System = 1,
    }
}
