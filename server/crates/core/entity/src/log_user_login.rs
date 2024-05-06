//! 用户登录日志表

use chrono::Local;
use sea_orm::{
    prelude::{async_trait::async_trait, DateTimeLocal},
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Set,
};
use serde::{Deserialize, Serialize};

/// 用户登录日志表
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_log_user_login")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 用户名称
    pub username: String,
    /// 登陆令牌
    pub token: String,
    /// 登录IP
    pub remote_addr: String,
    /// 用户代理
    pub user_agent: String,
    /// 设备
    pub device: Option<String>,
    /// 系统
    pub system: Option<String>,
    /// 浏览器
    pub browser: Option<String>,
    /// 登录状态,0:失败,1:成功
    pub status: i8,
    /// 禁用状态,0:未禁用,1:禁用
    pub disabled: i8,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Local::now());
        Ok(self)
    }
}

/// 枚举
pub mod enums {
    use serde_repr::{Deserialize_repr, Serialize_repr};

    /// 用户登陆状态
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Status {
        /// 失败
        Failed = 0,
        /// 成功
        Success = 1,
    }

    /// 用户登陆禁用状态
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum DisabledStatus {
        /// 停用
        Disabled = 0,
        /// 正常
        Enabled = 1,
    }
}
