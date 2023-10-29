//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "api_http")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
    pub method: String,
    pub uri: String,
    pub note: Option<String>,
    pub status: i8,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::api_role_http_rel::Entity")]
    ApiRoleHttpRel,
}

impl Related<super::api_role_http_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiRoleHttpRel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
