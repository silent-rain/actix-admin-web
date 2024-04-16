//! WEB日志表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// WEB日志表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_log_web")]
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
    /// 终端类型: 0: 未知,1: 安卓,2 :ios,3 :web
    pub os_type: i8,
    /// 错误类型: 1:接口报错,2:代码报错
    pub error_type: i8,
    /// 日志级别
    pub level: String,
    /// 日发生位置
    pub caller_line: String,
    /// 错误页面
    pub url: String,
    /// 日志消息
    #[sea_orm(column_type = "Text", nullable)]
    pub msg: Option<String>,
    /// 堆栈信息
    #[sea_orm(column_type = "Text", nullable)]
    pub stack: Option<String>,
    /// 备注
    pub note: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
