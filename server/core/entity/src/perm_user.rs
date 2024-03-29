//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "perm_user")]
pub struct Model {
    #[serde(default)]
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: Option<String>,
    pub gender: i8,
    pub age: Option<i32>,
    pub birthday: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub intro: Option<String>,
    pub note: Option<String>,
    pub password: String,
    pub status: i8,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::api_token::Entity")]
    ApiToken,
    #[sea_orm(has_many = "super::perm_user_role_rel::Entity")]
    PermUserRoleRel,
}

impl Related<super::api_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiToken.def()
    }
}

impl Related<super::perm_user_role_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermUserRoleRel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
