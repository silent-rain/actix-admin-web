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

/// 库表数据
#[derive(Serialize, Clone, Deserialize)]
pub struct TableDataSql {
    /// 角色表
    pub role_sql: String,
    /// OpenAPi表
    pub openapi_sql: String,
    /// 菜单表
    pub menu_sql: String,
    /// 任务调度作业表
    pub schedule_job_sql: String,
}
