//! 常量

/// 系统鉴权标识
pub const SYSTEM_API_AUTHORIZATION: &str = "Authorization";
/// 系统鉴权标识-前缀
pub const SYSTEM_API_AUTHORIZATION_BEARER: &str = "Bearer ";

/// OPEN API鉴权标识
pub const OPENAPI_AUTHORIZATION: &str = "X-SR-Token";
/// OPEN API鉴权口令
pub const OPENAPI_PASSPHRASE: &str = "X-SR-Passphrase";

/// 请求白名单
pub const AUTH_WHITE_LIST: [&str; 5] = [
    "/api/v1/health",
    "/api/v1/auth/captcha",
    "/api/v1/auth/login",
    "/api/v1/auth/register",
    "/api/v1/initialize/table",
];
