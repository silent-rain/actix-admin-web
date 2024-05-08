//! 菜单表
use utils::list_tree::GenericTreeTrait;

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 菜单表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_menu")]
pub struct Model {
    /// 菜单ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 父菜单ID
    pub pid: Option<i32>,
    /// 菜单名称
    pub title: String,
    /// Icon图标
    pub icon: Option<String>,
    /// Element-Icon图标
    pub el_icon: Option<String>,
    /// 菜单类型(0:菜单,1:按钮)
    pub menu_type: i8,
    /// 打开方式(0:组件,1:内链,2:外链)
    pub open_type: i8,
    /// 路由地址
    pub path: Option<String>,
    /// 组件路径
    pub component: Option<String>,
    /// 路由重定向
    pub redirect: Option<String>,
    /// 链接地址:站内链地址/站外链地址
    pub link: Option<String>,
    /// 链接跳转方式, _blank/_self
    pub link_target: Option<String>,
    /// 是否隐藏(0:显示,1:隐藏)
    pub hidden: Option<i8>,
    /// 始终显示根菜单(0:显示,1:隐藏)
    pub root_always_show: Option<i8>,
    /// 权限标识
    pub permission: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(0:停用,1:正常)
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

/// 实现 `GenericTreeTrait` trait, 将列表数据转换为树结构
impl GenericTreeTrait for Model {
    fn id(&self) -> i32 {
        self.id
    }

    fn pid(&self) -> Option<i32> {
        self.pid
    }
}

/// 枚举
pub mod enums {
    use serde::{Deserialize, Serialize};
    use serde_repr::{Deserialize_repr, Serialize_repr};

    /// 菜单状态
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Status {
        /// 停用
        Disabled = 0,
        /// 正常
        Enabled = 1,
    }

    /// 菜单类型
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum MenuType {
        /// 菜单
        Menu = 0,
        /// 按钮
        Button = 1,
    }

    /// 菜单打开方式
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum OpenType {
        /// 组件
        Component = 0,
        /// 内链
        InternalLink = 1,
        /// 外链
        ExternalLink = 2,
    }

    /// 菜单链接跳转方式
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum LinkTarget {
        /// 新窗口中打开
        #[serde(rename = "_blank")]
        Blank,
        /// 当前窗口中打开
        #[serde(rename = "_self")]
        Current,
    }

    impl From<LinkTarget> for String {
        fn from(value: LinkTarget) -> Self {
            match value {
                LinkTarget::Blank => "_blank".to_owned(),
                LinkTarget::Current => "_self".to_owned(),
            }
        }
    }

    /// 菜单是否隐藏
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Hidden {
        /// 显示
        Visible = 0,
        /// 隐藏
        Hidden = 1,
    }

    /// 始终显示根菜单
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum RootAlwaysShow {
        /// 显示
        Show = 0,
        /// 隐藏
        Hide = 1,
    }
}
