//! 岗位表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 岗位表
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_org_position")]
pub struct Model {
    /// 岗位ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 岗位名称
    #[sea_orm(unique)]
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 岗位描述
    pub desc: Option<String>,
    /// 所属部门ID
    pub department_id: Option<i32>,
    /// 状态(0:停用,1:正常)
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

    /// 状态
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Status {
        /// 停用
        Disabled = 0,
        /// 正常
        Enabled = 1,
    }
}
