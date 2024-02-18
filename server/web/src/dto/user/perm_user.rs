//! 用户管理
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 用户列表查询
#[derive(Default, Deserialize)]
pub struct UserListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
}

/// 通过 ID 查询用户详情信息
#[derive(Default, Deserialize)]
pub struct UserInfoReq {
    pub id: i32,
}

/// 添加用户
#[derive(Serialize, Deserialize, Validate)]
pub struct AddUserReq {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub username: String,
    pub gender: i8,
    #[validate(range(min = 18, max = 22, message = "Age must be between 18 to 22"))]
    pub age: i32,
    pub birth: Option<String>,
    pub phone: String,
    #[validate(
        email,
        contains(pattern = "gmail", message = "Email must be valid gmail address")
    )]
    pub email: Option<String>,
    pub password: String,
    pub password2: String,
    pub avatar: Option<String>,
}

/// 删除用户
#[derive(Default, Deserialize)]
pub struct DeleteUserReq {
    pub id: i32,
}
