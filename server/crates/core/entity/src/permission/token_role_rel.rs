//! 令牌角色关系表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 令牌角色关系表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_token_role_rel")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 令牌ID
    pub token_id: i32,
    /// 角色ID
    pub role_id: i32,
    /// 创建时间
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::token::Entity",
        from = "Column::TokenId",
        to = "super::token::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PermToken,
    #[sea_orm(
        belongs_to = "crate::user::role::Entity",
        from = "Column::RoleId",
        to = "crate::user::role::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    UserRole,
}

impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermToken.def()
    }
}

impl Related<super::super::user::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
