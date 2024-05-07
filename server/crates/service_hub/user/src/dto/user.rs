//! 用户管理

use entity::perm_user;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询用户列表
#[derive(Default, Deserialize, Validate)]
pub struct GetUserListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户名称
    pub username: Option<String>,
}

/// 添加用户
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct AddUserReq {
    /// 用户名称
    pub username: String,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别, 0:男,1:女,2:保密
    pub gender: perm_user::enums::Gender,
    /// 年龄
    pub age: i32,
    /// 出生日期
    pub birthday: Option<String>,
    /// 密码
    pub password: String,
    /// 头像URL
    pub avatar: Option<String>,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

/// 更新用户
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserReq {
    /// 用户名称
    pub username: String,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别, 0:男,1:女,2:保密
    pub gender: perm_user::enums::Gender,
    /// 年龄
    pub age: i32,
    /// 出生日期
    pub birthday: Option<String>,
    /// 密码
    pub password: String,
    /// 头像URL
    pub avatar: Option<String>,
    /// 介绍
    pub intro: Option<String>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    pub status: perm_user::enums::Status,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

/// 更新数据状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserStatusReq {
    /// 用户状态
    pub status: perm_user::enums::Status,
}

/// 获取用户个人信息
#[derive(Clone, Serialize, Deserialize)]
pub struct ProfileRsp {
    /// 用户ID
    pub id: i32,
    /// 用户名称
    pub username: String,
    /// 性别
    pub gender: i8,
    /// 年龄
    pub age: Option<i32>,
    /// 出生日期
    pub birthday: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
}