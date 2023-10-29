//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "perm_role_menu_rel")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role_id: i32,
    pub menu_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::perm_menu::Entity",
        from = "Column::MenuId",
        to = "super::perm_menu::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PermMenu,
    #[sea_orm(
        belongs_to = "super::perm_role::Entity",
        from = "Column::RoleId",
        to = "super::perm_role::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PermRole,
}

impl Related<super::perm_menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermMenu.def()
    }
}

impl Related<super::perm_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
