//! 注册
use crate::{
    common::captcha::check_captcha, dao::register::RegisterDao, dto::register::RegisterReq,
};

use permission::{UserEmailDao, UserPhoneDao};
use system::CaptchaDao;

use code::{Error, ErrorMsg};
use entity::perm_user;
use utils::crypto::sha2_256;

use nject::injectable;
use tracing::error;

/// 服务层
#[injectable]
pub struct RegisterService<'a> {
    user_email_dao: UserEmailDao<'a>,
    user_phone_dao: UserPhoneDao<'a>,
    register_dao: RegisterDao<'a>,
    captcha_dao: CaptchaDao<'a>,
}

impl<'a> RegisterService<'a> {
    /// 根据不同的注册类型进行注册用户
    pub async fn register(&self, data: RegisterReq) -> Result<perm_user::Model, ErrorMsg> {
        // 检测验证码
        check_captcha(
            &self.captcha_dao,
            data.captcha_id.clone(),
            data.captcha.clone(),
        )
        .await?;

        // 根据不同注册类型进行注册
        match data.register_type {
            perm_user::enums::UserType::Phone => self.register_phone(data).await,
            perm_user::enums::UserType::Email => self.register_email(data).await,
        }
    }

    /// 注册手机用户
    async fn register_phone(&self, data: RegisterReq) -> Result<perm_user::Model, ErrorMsg> {
        let mut data = data.clone();

        let phone = match data.phone.clone() {
            Some(v) => v,
            None => {
                return Err(code::Error::InvalidParameterError
                    .into_msg()
                    .with_msg("请求参数错误, phone 不能为空"))
            }
        };

        // TODO 检测手机验证码, 待接入第三方服务

        // 检测是否已注册用户
        let user = self
            .user_phone_dao
            .info_by_phone(phone)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?;
        if user.is_some() {
            {
                error!("该手机号码已注册");
                return Err(code::Error::DbDataExistError
                    .into_msg()
                    .with_msg("该手机号码已注册"));
            };
        }

        // 密码加密
        data.password = sha2_256(&data.password);

        // 添加用户
        let result = self.register_dao.add_user(data).await.map_err(|err| {
            error!("注册手机用户失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("注册手机用户失败")
        })?;

        Ok(result)
    }

    /// 注册邮箱用户
    async fn register_email(&self, data: RegisterReq) -> Result<perm_user::Model, ErrorMsg> {
        let mut data = data.clone();

        let email = match data.email.clone() {
            Some(v) => v,
            None => {
                return Err(code::Error::DbDataExistError
                    .into_msg()
                    .with_msg("请求参数错误, email 不能为空"))
            }
        };

        // 检测是否已注册邮箱
        let user = self
            .user_email_dao
            .info_by_email(email)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?;
        if user.is_some() {
            {
                error!("该邮箱已注册");
                return Err(code::Error::DbDataExistError
                    .into_msg()
                    .with_msg("该邮箱已注册"));
            };
        }

        // 密码加密
        data.password = sha2_256(&data.password);

        // 添加用户
        let result = self.register_dao.add_user(data).await.map_err(|err| {
            error!("注册邮箱用户失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("注册邮箱用户失败")
        })?;

        // TODO 邮箱验证, 发送链接点击后确认

        Ok(result)
    }
}
