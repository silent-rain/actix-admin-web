//! WEB日志表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait,
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
    /// 终端类型
    pub os_type: i8,
    /// 错误类型
    pub error_type: i8,
    /// 日志级别
    pub level: String,
    /// 日发生位置
    pub caller_line: String,
    /// 错误页面
    pub url: Option<String>,
    /// 日志消息
    #[sea_orm(column_type = "Text", nullable)]
    pub msg: Option<String>,
    /// 堆栈信息
    #[sea_orm(column_type = "Text", nullable)]
    pub stack: Option<String>,
    /// 描述信息
    pub desc: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// 枚举
pub mod enums {
    use serde_repr::{Deserialize_repr, Serialize_repr};

    /// 终端类型(0:未知, 1:安卓, 2:IOS, 3:WEB)
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum OsType {
        /// 未知
        Unknown = 0,
        /// 安卓
        Android = 1,
        /// IOS
        IOS = 2,
        /// WEB
        Web = 3,
    }

    /// 错误类型(0:代码报错, 1:接口报错)
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum ErrorType {
        CodeError = 0,
        InterfaceError = 1,
    }
}
