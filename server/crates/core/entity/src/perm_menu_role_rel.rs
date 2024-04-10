//! 菜单角色关联表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 菜单角色关联表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "perm_menu_role_rel")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 角色ID
    pub role_id: i32,
    /// 菜单ID
    pub menu_id: i32,
    /// 创建者
    pub creator: Option<i32>,
    /// 创建时间
    pub created_at: DateTimeLocal,
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
