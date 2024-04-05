//! 验证码

use crate::app::system::CaptchaDao;

use code::{Error, ErrorMsg};

use chrono::Local;
use tracing::error;

/// 检测验证码
pub async fn check_captcha<'a>(
    captcha_dao: &'a CaptchaDao<'a>,
    captcha_id: String,
    captcha: String,
) -> Result<(), ErrorMsg> {
    let result = captcha_dao
        .info_by_captcha_id(captcha_id)
        .await
        .map_err(|err| {
            error!("查询验证码失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询验证码失败")
        })?
        .ok_or_else(|| {
            error!("验证码信息不存在, captcha: {}", captcha);
            Error::DbQueryEmptyError
                .into_msg()
                .with_msg("验证码信息不存在")
        })?;

    // 验证验证码
    if result.captcha.to_uppercase() != captcha.to_uppercase() {
        return {
            error!("验证码错误, captcha: {}", captcha);
            Err(Error::CaptchaInvalid.into_msg().with_msg("验证码错误"))
        };
    }

    // 验证过期时间
    let max_time = result.created_at.timestamp() + result.expire as i64;
    let now = Local::now().timestamp();
    if now > max_time {
        return {
            error!("验证码过期, captcha: {}, max_time: {}", captcha, max_time);
            Err(Error::CaptchaExpire.into_msg().with_msg("验证码过期"))
        };
    }

    Ok(())
}
