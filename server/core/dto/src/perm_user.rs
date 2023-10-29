//!用户管理
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct AddUserReq {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub realname: String,
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub nickname: String,
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
