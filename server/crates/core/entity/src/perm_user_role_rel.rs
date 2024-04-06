//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "perm_user_role_rel")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::perm_role::Entity",
        from = "Column::RoleId",
        to = "super::perm_role::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PermRole,
    #[sea_orm(
        belongs_to = "super::perm_user::Entity",
        from = "Column::UserId",
        to = "super::perm_user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PermUser,
}

impl Related<super::perm_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermRole.def()
    }
}

impl Related<super::perm_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}