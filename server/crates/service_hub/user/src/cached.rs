//! 缓存
use core::time;

use cache::Cache;
use code::Error;

use crate::dto::user_base::UserPermission;

/// 系统用户鉴权KEY
pub const USER_SYSTEM_API_AUTH: &str = "USER_SYSTEM_API_AUTH";

/// Openapi用户鉴权KEY
pub const USER_OPENAPI_API_AUTH: &str = "USER_OPENAPI_API_AUTH";

/// 用户接口权限缓存过期时间
pub const USER_EXPIRY: u64 = 1000 * 60 * 60 * 24;

/// 用户管理缓存
pub struct UserCached;

impl UserCached {
    /// 设置系统用户鉴权
    pub async fn set_user_system_api_auth(user_id: i32, data: UserPermission) {
        Cache::default()
            .set_with_expiry(
                &format!("{}_{}", USER_SYSTEM_API_AUTH, user_id),
                data,
                time::Duration::from_millis(USER_EXPIRY),
            )
            .await;
    }
    /// 获取系统用户鉴权
    pub async fn get_user_system_api_auth(user_id: i32) -> Result<UserPermission, Error> {
        let result = Cache::default()
            .get_with_expiry(&format!("{}_{}", USER_SYSTEM_API_AUTH, user_id))
            .await;
        let result = match result {
            Some(v) => v.value,
            None => return Err(Error::CacheNotFound),
        };
        let permission: UserPermission =
            serde_json::from_value(result).map_err(|_err| Error::JsonConvert)?;
        Ok(permission)
    }

    /// 设置Openapi用户鉴权
    pub async fn set_user_openapi_api_auth(openapi_token: String, data: UserPermission) {
        Cache::default()
            .set_with_expiry(
                &format!("{}_{}", USER_OPENAPI_API_AUTH, openapi_token),
                data,
                time::Duration::from_millis(USER_EXPIRY),
            )
            .await;
    }
    /// 获取Openapi用户鉴权
    pub async fn get_user_openapi_api_auth(openapi_token: String) -> Result<UserPermission, Error> {
        let result = Cache::default()
            .get_with_expiry(&format!("{}_{}", USER_OPENAPI_API_AUTH, openapi_token))
            .await;
        let result = match result {
            Some(v) => v.value,
            None => return Err(Error::CacheNotFound),
        };
        let permission: UserPermission =
            serde_json::from_value(result).map_err(|_err| Error::JsonConvert)?;
        Ok(permission)
    }
}
