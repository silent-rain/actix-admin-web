//! 菜单表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 菜单表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "perm_menu")]
pub struct Model {
    /// 菜单ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 父菜单ID
    pub parent_id: Option<i32>,
    /// 菜单名称
    pub title: String,
    /// 菜单图标
    pub icon: Option<String>,
    /// Element菜单图标
    pub el_svg_icon: Option<String>,
    /// 菜单类型,0:菜单,1:按钮
    pub menu_type: i8,
    /// 打开方式,0:组件,1:内链,2:外链
    pub open_type: i8,
    /// 路由地址
    pub path: Option<String>,
    /// 路由别名
    pub name: Option<String>,
    /// 组件路径
    pub component: Option<String>,
    /// 路由重定向
    pub redirect: Option<String>,
    /// 链接地址:内链地址/外链地址
    pub link: Option<String>,
    /// 链接地址跳转方式, component/_blank/_self
    pub target: Option<String>,
    /// 权限标识
    pub permission: Option<String>,
    /// 是否隐藏,0:显示,1:隐藏
    pub hidden: Option<i8>,
    /// 始终显示根菜单,0:显示,1:隐藏
    pub always_show: Option<i8>,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:启用
    pub status: i8,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::perm_menu_role_rel::Entity")]
    PermMenuRoleRel,
}

impl Related<super::perm_menu_role_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermMenuRoleRel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
