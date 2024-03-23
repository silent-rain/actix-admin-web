//! 用户角色关联关系管理
use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 用户与角色关联关系 结构
#[derive(Default, Deserialize, Validate)]
pub struct UserRoleRel {
    pub user_id: i32,
    pub role_id: i32,
    pub role_name: i32,
}

/// 查询用户与角色关联关系
#[derive(Default, Deserialize, Validate)]
pub struct GetUserRoleRelListReq {
    pub user_id: i32,
}

/// 添加用户与角色关联关系请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct AddUserRoleRelReq {
    pub user_id: i32,
    pub role_id: i32,
}

/// 删除用户与角色关联关系请求体
#[derive(Default, Deserialize, Validate)]
pub struct DeleteUserRoleRelReq {
    pub user_id: i32,
}
