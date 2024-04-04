//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "perm_menu")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub parent_id: Option<i32>,
    pub title: String,
    pub icon: Option<String>,
    pub el_svg_icon: Option<String>,
    pub menu_type: i8,
    pub open_type: i8,
    pub path: Option<String>,
    pub name: Option<String>,
    pub component: Option<String>,
    pub redirect: Option<String>,
    pub link: Option<String>,
    pub target: Option<String>,
    pub permission: Option<String>,
    pub hidden: Option<i8>,
    pub always_show: Option<i8>,
    pub sort: i32,
    pub note: Option<String>,
    pub status: i8,
    pub create_user_id: Option<i32>,
    pub update_user_id: Option<i32>,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::perm_role_menu_rel::Entity")]
    PermRoleMenuRel,
}

impl Related<super::perm_role_menu_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermRoleMenuRel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
