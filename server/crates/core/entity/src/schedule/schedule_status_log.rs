//! 任务调度状态日志表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 任务调度状态日志表
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_schedule_status_log")]
pub struct Model {
    /// 状态日志ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 任务ID
    pub job_id: i32,
    /// 任务调度ID
    pub uuid: String,
    /// 失败信息
    pub error: Option<String>,
    /// 耗时,毫秒
    pub cost: u64,
    /// 任务状态
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

    /// 定时任务事件状态
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Status {
        /// 运行中
        Running = 0,
        /// 完成
        Completed = 1,
        /// 失败
        Failed = 2,
    }
}
