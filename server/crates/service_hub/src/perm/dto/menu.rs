//! 菜单管理

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询菜单列表
#[derive(Default, Deserialize, Validate)]
pub struct GetMenuListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 菜单名称
    pub title: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 添加菜单
#[derive(Serialize, Deserialize, Validate)]
pub struct AddMenuReq {
    /// 父菜单ID
    pub pid: Option<i32>,
    /// 菜单名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub title: String,
    /// Icon图标
    pub icon: Option<String>,
    /// Element-Icon图标
    pub el_icon: Option<String>,
    /// 菜单类型,0:菜单,1:按钮
    /// Enum: [`crate::perm::enums::MenuType`]
    pub menu_type: i8,
    /// 打开方式,0:组件,1:内链,2:外链
    /// Enum: [`crate::perm::enums::MenuOpenType`]
    pub open_type: i8,
    /// 路由地址
    pub path: Option<String>,
    /// 组件路径
    pub component: Option<String>,
    /// 路由重定向
    pub redirect: Option<String>,
    /// 链接地址:站内链地址/站外链地址
    /// Enum: [`crate::perm::enums::MenuLinkType`]
    pub link: Option<String>,
    /// 链接跳转方式, _blank/_self
    /// Enum: [`crate::perm::enums::MenuLinkTarget`]
    pub link_target: Option<String>,
    /// 是否隐藏,0:显示,1:隐藏
    /// Enum: [`crate::perm::enums::MenuHidden`]
    pub hidden: Option<i8>,
    /// 始终显示根菜单,0:显示,1:隐藏
    /// Enum: [`crate::perm::enums::MenuRootAlwaysShow`]
    pub root_always_show: Option<i8>,
    /// 权限标识
    pub permission: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    /// Enum: [`crate::perm::enums::MenuStatus`]
    pub status: i8,
}

/// 更新数据
#[derive(Default, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMenuReq {
    /// 父菜单ID
    pub pid: Option<i32>,
    /// 菜单名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub title: String,
    /// Icon图标
    pub icon: Option<String>,
    /// Element-Icon图标
    pub el_icon: Option<String>,
    /// 菜单类型,0:菜单,1:按钮
    pub menu_type: i8,
    /// 打开方式,0:组件,1:内链,2:外链
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
    /// 是否隐藏,0:显示,1:隐藏
    pub hidden: Option<i8>,
    /// 始终显示根菜单,0:显示,1:隐藏
    pub root_always_show: Option<i8>,
    /// 权限标识
    pub permission: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    /// Enum: [`crate::perm::enums::MenuStatus`]
    pub status: i8,
}

/// 更新数据状态
#[derive(Default, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMenuStatusReq {
    /// 状态,0:停用,1:正常
    /// Enum: [`crate::perm::enums::MenuStatus`]
    pub status: i8,
}
