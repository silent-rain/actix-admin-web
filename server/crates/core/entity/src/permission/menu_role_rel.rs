//! 菜单角色关系表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 菜单角色关系表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_menu_role_rel")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 菜单ID
    pub menu_id: i32,
    /// 角色ID
    pub role_id: i32,
    /// 创建时间
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::menu::Entity",
        from = "Column::MenuId",
        to = "super::menu::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PermMenu,
    #[sea_orm(
        belongs_to = "crate::user::role::Entity",
        from = "Column::RoleId",
        to = "crate::user::role::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    UserRole,
}

impl Related<crate::permission::menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermMenu.def()
    }
}

impl Related<crate::user::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
