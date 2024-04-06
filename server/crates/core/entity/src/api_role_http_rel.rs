//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "api_role_http_rel")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role_id: i32,
    pub api_id: i32,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::api_http::Entity",
        from = "Column::ApiId",
        to = "super::api_http::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ApiHttp,
    #[sea_orm(
        belongs_to = "super::perm_role::Entity",
        from = "Column::RoleId",
        to = "super::perm_role::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PermRole,
}

impl Related<super::api_http::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiHttp.def()
    }
}

impl Related<super::perm_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}