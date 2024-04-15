//! 定时任务日志表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 定时任务日志表
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "log_schedule_job")]
pub struct Model {
    /// 日志ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 任务ID
    pub job_id: i32,
    /// 任务名称
    pub job_name: String,
    /// 失败信息
    pub error: Option<String>,
    /// 耗时(单位：毫秒)
    pub cost: f64,
    /// 任务状态,0:失败,1:成功'
    pub status: i8,
    /// 创建时间
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
