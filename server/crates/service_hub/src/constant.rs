//! 常量

/// 系统鉴权标识
pub const HEADERS_AUTHORIZATION: &str = "Authorization";
/// 系统鉴权标识-前缀
pub const HEADERS_AUTHORIZATION_BEARER: &str = "Bearer ";

/// OPEN API鉴权标识
pub const HEADERS_OPEN_API_AUTHORIZATION: &str = "X-SR-Token";

/// 验证码过期时间
pub const CAPTCHA_EXPIRE: u32 = 120;

/// 图片标识
pub const HEADERS_X_IMG: &str = "X-SR-IMG";
