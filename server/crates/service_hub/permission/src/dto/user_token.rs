//! 用户Token令牌管理

use actix_validator::Validate;
use utils::time::{default_local_date_time, str_to_local_date_time};

use sea_orm::prelude::DateTimeLocal;
use serde::{Deserialize, Serialize};

use crate::enums::UserTokenStatus;

/// 查询用户令牌列表
#[derive(Default, Deserialize, Validate)]
pub struct GetUserTokenListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户ID
    pub user_id: Option<i32>,
    /// 令牌
    pub token: Option<String>,
}

/// 添加用户令牌
#[derive(Serialize, Deserialize, Validate)]
pub struct AddUserTokenReq {
    /// 用户ID
    pub user_id: i32,
    /// 权限范围:GET,POST,PUT,DELETE
    /// Enum: [`crate::enums::UserTokenPermission`]
    pub permission: String,
    /// 授权到期时间
    #[serde(
        rename = "expire",
        deserialize_with = "str_to_local_date_time",
        default = "default_local_date_time"
    )]
    pub expire: DateTimeLocal,
    /// 状态,0:禁用,1:启用
    pub status: UserTokenStatus,
    /// 备注
    pub note: Option<String>,
}

/// 更新数据
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserTokenReq {
    /// 用户ID
    pub user_id: i32,
    /// 权限范围:GET,POST,PUT,DELETE
    /// Enum: [`crate::enums::UserTokenPermission`]
    pub permission: String,
    /// 授权到期时间
    #[serde(
        rename = "expire",
        deserialize_with = "str_to_local_date_time",
        default = "default_local_date_time"
    )]
    pub expire: DateTimeLocal,
    /// 状态,0:禁用,1:启用
    pub status: UserTokenStatus,
    /// 备注
    pub note: Option<String>,
}

/// 更新用户令牌状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserTokenStatusReq {
    /// 状态,0:停用,1:正常
    pub status: UserTokenStatus,
}
