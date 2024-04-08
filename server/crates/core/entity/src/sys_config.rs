//! 应用配置表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 应用配置表
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "sys_config")]
pub struct Model {
    /// 配置ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 父节点ID
    pub pid: Option<i32>,
    /// 配置名称
    pub name: String,
    /// 配置项(英文)
    #[sea_orm(unique)]
    pub key: String,
    /// 配置参数值
    #[sea_orm(column_type = "Text", nullable)]
    pub value: Option<String>,
    /// 排序
    pub sort: i32,
    /// 配置描述
    pub note: Option<String>,
    /// 是否启用,0: 禁用,1: 启用
    pub status: i8,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
