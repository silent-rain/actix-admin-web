//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "log_web")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: Option<i32>,
    pub nickname: Option<String>,
    pub trace_id: Option<String>,
    pub os_type: i8,
    pub error_type: i8,
    pub level: String,
    pub caller_line: String,
    pub url: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub msg: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub stack: Option<String>,
    pub note: Option<String>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}