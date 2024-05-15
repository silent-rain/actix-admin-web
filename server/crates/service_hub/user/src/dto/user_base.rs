//! 用户信息管理

use entity::{perm_openapi, user::user_base};

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询用户列表
#[derive(Default, Deserialize, Validate)]
pub struct GetUserBaserListReq {
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
pub struct AddUserBaseReq {
    /// 用户名称
    pub username: String,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别(0:男,1:女,2:保密)
    pub gender: user_base::enums::Gender,
    /// 密码
    pub password: String,
    /// 状态(0:停用,1:正常)
    pub status: user_base::enums::Status,
    /// 年龄
    pub age: Option<i32>,
    /// 出生日期
    pub date_birth: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 用户个人介绍
    pub intro: Option<String>,
    /// 用户描述
    pub desc: Option<String>,
    /// 用户的居住或邮寄地址
    pub address: Option<String>,
    /// 偏好设置
    pub preferences: Option<String>,
    /// 所属部门ID
    pub department_id: Option<i32>,
    /// 所属岗位ID
    pub position_id: Option<i32>,
    /// 所属职级ID
    pub rank_id: Option<i32>,
    /// 用户会员等级ID
    pub member_level_id: Option<i32>,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

/// 更新用户
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserBaseReq {
    /// 用户名称
    pub username: String,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别(0:男,1:女,2:保密)
    pub gender: user_base::enums::Gender,
    /// 状态(0:停用,1:正常)
    pub status: user_base::enums::Status,
    /// 年龄
    pub age: Option<i32>,
    /// 出生日期
    pub date_birth: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 用户个人介绍
    pub intro: Option<String>,
    /// 用户描述
    pub desc: Option<String>,
    /// 用户的居住或邮寄地址
    pub address: Option<String>,
    /// 偏好设置
    pub preferences: Option<String>,
    /// 所属部门ID
    pub department_id: Option<i32>,
    /// 所属岗位ID
    pub position_id: Option<i32>,
    /// 所属职级ID
    pub rank_id: Option<i32>,
    /// 用户会员等级ID
    pub member_level_id: Option<i32>,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

/// 更新数据状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserBaseStatusReq {
    /// 用户状态
    pub status: user_base::enums::Status,
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
    pub date_birth: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
}

/// 用户接口权限权限
#[derive(Clone, Serialize, Deserialize)]
pub struct UserPermission {
    pub user_id: i32,
    pub username: String,
    pub role_ids: Vec<i32>,
    pub openapis: Vec<perm_openapi::Model>,
}
