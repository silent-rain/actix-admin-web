//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "perm_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub sort: i32,
    pub note: Option<String>,
    pub status: i8,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::api_role_http_rel::Entity")]
    ApiRoleHttpRel,
    #[sea_orm(has_many = "super::perm_role_menu_rel::Entity")]
    PermRoleMenuRel,
    #[sea_orm(has_many = "super::perm_user_role_rel::Entity")]
    PermUserRoleRel,
}

impl Related<super::api_role_http_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiRoleHttpRel.def()
    }
}

impl Related<super::perm_role_menu_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermRoleMenuRel.def()
    }
}

impl Related<super::perm_user_role_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermUserRoleRel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
