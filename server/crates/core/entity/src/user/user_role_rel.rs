//! 用户角色关系表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 用户角色关系表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_user_role_rel")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 角色ID
    pub role_id: i32,
    /// 创建时间
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::user::role::Entity",
        from = "Column::RoleId",
        to = "crate::user::role::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    UserRole,
    #[sea_orm(
        belongs_to = "crate::user::user_base::Entity",
        from = "Column::UserId",
        to = "crate::user::user_base::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    UserBase,
}

impl Related<crate::user::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl Related<crate::user::user_base::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBase.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
