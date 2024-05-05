//! 库表初始化

use actix_validator::Validate;
use serde::{Deserialize, Serialize};

/// 添加管理员用户
#[derive(Serialize, Clone, Deserialize, Validate)]
pub struct AddAdminUserReq {
    /// 用户名称
    pub username: String,
    /// 手机号码
    pub phone: String,
    /// 邮箱
    pub email: Option<String>,
    /// 密码
    pub password: String,
}

/// 数据库SQL
#[derive(Serialize, Clone, Deserialize)]
pub struct TableSql {
    /// 数据库
    pub db_sql: String,
    /// 角色表
    pub role_sql: String,
    /// OpenAPi表
    pub open_api_sql: String,
    /// 菜单表
    pub menu_sql: String,
}
