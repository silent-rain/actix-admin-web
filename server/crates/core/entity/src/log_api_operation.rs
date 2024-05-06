//! API操作日志表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// API操作日志表
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_log_api_operation")]
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
    /// 耗时,毫秒
    pub cost: u64,
    /// 请求类型:REQ/RSP
    pub http_type: String,
    /// 备注
    pub note: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// 枚举
pub mod enums {
    use serde::{Deserialize, Serialize};

    /// Api 操作日志类型
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum HttpType {
        /// 请求
        #[serde(rename = "REQ")]
        Req,
        /// 响应
        #[serde(rename = "RSP")]
        Rsp,
    }

    impl From<HttpType> for String {
        fn from(value: HttpType) -> Self {
            match value {
                HttpType::Req => "REQ".to_owned(),
                HttpType::Rsp => "RSP".to_owned(),
            }
        }
    }
}
