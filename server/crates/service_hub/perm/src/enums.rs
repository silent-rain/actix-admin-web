//! 枚举
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 角色状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum RoleStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 用户状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum UserStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 性别
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum Gender {
    /// 男
    Male = 0,
    /// 女
    Female = 1,
    /// 保密
    Confidential = 2,
}

/// 部门状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum DeptStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// OpenApi接口状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum OpenApiStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// OpenApi接口类别
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum OpenApiCategory {
    /// 目录
    Directory = 0,
    /// 接口
    Interface = 1,
}

/// 菜单状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum MenuStatus {
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
pub enum MenuOpenType {
    /// 组件
    Component = 0,
    /// 内链
    InternalLink = 1,
    /// 外链
    ExternalLink = 2,
}

/// 菜单链接跳转方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MenuLinkTarget {
    /// 新窗口中打开
    #[serde(rename = "_blank")]
    Blank,
    /// 当前窗口中打开
    #[serde(rename = "_self")]
    Current,
}

impl From<MenuLinkTarget> for String {
    fn from(value: MenuLinkTarget) -> Self {
        match value {
            MenuLinkTarget::Blank => "_blank".to_owned(),
            MenuLinkTarget::Current => "_self".to_owned(),
        }
    }
}

/// 菜单是否隐藏
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum MenuHidden {
    /// 显示
    Visible = 0,
    /// 隐藏
    Hidden = 1,
}

/// 始终显示根菜单
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum MenuRootAlwaysShow {
    /// 显示
    Show = 0,
    /// 隐藏
    Hide = 1,
}

/// 用户令牌状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum UserTokenStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 用户令牌权限范围
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserTokenPermission {
    /// 读取数据
    #[serde(rename = "GET")]
    GET,
    /// 提交数据
    #[serde(rename = "POST")]
    POST,
    /// 更新数据
    #[serde(rename = "PUT")]
    PUT,
    /// 删除数据
    #[serde(rename = "DELETE")]
    DELETE,
}
