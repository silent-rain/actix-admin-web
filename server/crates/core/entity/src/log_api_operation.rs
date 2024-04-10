//! API操作日志表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// API操作日志表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "log_api_operation")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: Option<i32>,
    /// 用户名称
    pub username: Option<String>,
    /// 请求ID
    pub request_id: Option<String>,
    /// 请求状态码
    pub status_code: i32,
    /// 请求方法
    pub method: String,
    /// 请求地址路径
    pub path: String,
    /// 请求参数
    pub query: Option<String>,
    /// 请求体/响应体
    #[sea_orm(column_type = "custom(\"LONGTEXT\")", nullable)]
    pub body: Option<String>,
    /// 请求IP
    pub remote_addr: String,
    /// 用户代理
    pub user_agent: String,
    /// 耗时,纳秒
    pub cost: i32,
    /// 请求类型:REQ/RSP
    pub htpp_type: String,
    /// 备注
    pub note: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
