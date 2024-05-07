//! 调度任务状态日志

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 调度任务状态日志
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_schedule_job_status_log")]
pub struct Model {
    /// 状态日志ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 任务ID
    pub job_id: i32,
    /// 调度任务ID
    pub uuid: String,
    /// 失败信息
    pub error: Option<String>,
    /// 耗时,毫秒
    pub cost: u64,
    /// 任务状态,0:开始,1:完成,2:停止,3:移除
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
        /// 开始
        Start = 0,
        /// 完成
        Done = 1,
        /// 停止
        Stop = 2,
        /// 移除
        Removed = 3,
    }
}
