//! 角色表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "perm_role")]
pub struct Model {
    /// 角色ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 角色名称
    #[sea_orm(unique)]
    pub name: String,
    /// 排序
    pub sort: i32,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    pub status: i8,
    /// 创建者
    pub creator: Option<i32>,
    /// 更新者
    pub updater: Option<i32>,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::api_role_http_rel::Entity")]
    ApiRoleHttpRel,
    #[sea_orm(has_many = "super::perm_role_menu_rel::Entity")]
    PermRoleMenuRel,
    #[sea_orm(has_many = "super::perm_role_user_rel::Entity")]
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

impl Related<super::perm_role_user_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermUserRoleRel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
