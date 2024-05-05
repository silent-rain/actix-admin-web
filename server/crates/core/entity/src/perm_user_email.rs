//! 用户邮箱表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 用户邮箱表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_user_email")]
pub struct Model {
    /// 邮箱ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 邮箱
    pub email: String,
    /// 备注
    pub note: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::perm_user::Entity")]
    PermUser,
}

impl Related<super::perm_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
