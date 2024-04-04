//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
    #[sea_orm(unique)]
    pub key: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub value: Option<String>,
    pub sort: i32,
    pub note: Option<String>,
    pub status: i8,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
