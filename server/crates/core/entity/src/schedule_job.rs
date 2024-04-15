//! 定时任务表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 定时任务表
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "schedule_job")]
pub struct Model {
    /// 定时任务ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 任务名称
    pub name: String,
    /// 任务类型,0:定时任务,1:即时任务
    pub job_type: i8,
    /// cron表达式
    pub expression: Option<String>,
    /// 间隔时间,秒
    pub interval: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 任务状态,0:暂停,1:正常
    pub status: i8,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
