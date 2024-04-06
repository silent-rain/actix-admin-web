//! 注册
use crate::{
    auth::common::captcha::check_captcha,
    auth::dto::register::{EmailRegisterReq, PhoneRegisterReq, RegisterReq, RegisterType},
    perm::UserDao,
    system::CaptchaDao,
};

use code::{Error, ErrorMsg};
use entity::perm_user;
use utils::{crypto::sha2_256, json::struct_to_struct};

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务
#[injectable]
pub struct RegisterService<'a> {
    user_dao: UserDao<'a>,
    captcha_dao: CaptchaDao<'a>,
}

impl<'a> RegisterService<'a> {
    /// 根据不同的注册类型进行注册用户
    pub async fn register(&self, data: RegisterReq) -> Result<perm_user::Model, ErrorMsg> {
        match data.register_type {
            RegisterType::Phone => {
                let data: PhoneRegisterReq = struct_to_struct(&data)?;
                self.register_phone(data).await
            }
            RegisterType::Email => {
                let data: EmailRegisterReq = struct_to_struct(&data)?;
                self.register_email(data).await
            }
        }
    }

    /// 注册手机用户
    async fn register_phone(&self, data: PhoneRegisterReq) -> Result<perm_user::Model, ErrorMsg> {
        let mut data = data;

        // 检测验证码
        check_captcha(
            &self.captcha_dao,
            data.captcha_id.clone(),
            data.captcha.clone(),
        )
        .await?;

        // TODO 检测手机验证码, 待接入第三方服务

        // 检测是否已注册用户
        let user = self
            .user_dao
            .info_by_phone(data.phone.clone())
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

        // 添加手机用户
        self.add_phone_user(data).await
    }

    /// 注册邮件用户
    async fn register_email(&self, data: EmailRegisterReq) -> Result<perm_user::Model, ErrorMsg> {
        let mut data = data;

        // 检测验证码
        check_captcha(
            &self.captcha_dao,
            data.captcha_id.clone(),
            data.captcha.clone(),
        )
        .await?;

        // 检测是否已注册邮件
        let user = self
            .user_dao
            .info_by_email(data.email.clone())
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

        // 添加邮箱用户
        let result = self.add_email_user(data).await?;

        // TODO 邮箱验证, 发送链接点击后确认

        Ok(result)
    }

    /// 添加手机用户
    async fn add_phone_user(&self, data: PhoneRegisterReq) -> Result<perm_user::Model, ErrorMsg> {
        let data = perm_user::ActiveModel {
            username: Set(Some(data.username)),
            gender: Set(data.gender),
            age: Set(data.age),
            birthday: Set(data.birthday),
            avatar: Set(data.avatar),
            phone: Set(Some(data.phone)),
            password: Set(data.password),
            status: Set(1),
            ..Default::default()
        };

        let result = self.user_dao.add_user(data, vec![]).await.map_err(|err| {
            error!("注册手机用户失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("注册手机用户失败")
        })?;
        Ok(result)
    }

    /// 添加邮箱用户
    async fn add_email_user(&self, data: EmailRegisterReq) -> Result<perm_user::Model, ErrorMsg> {
        let data = perm_user::ActiveModel {
            username: Set(Some(data.username)),
            gender: Set(data.gender),
            age: Set(data.age),
            birthday: Set(data.birthday),
            avatar: Set(data.avatar),
            email: Set(Some(data.email)),
            password: Set(data.password),
            status: Set(1),
            ..Default::default()
        };

        let result = self.user_dao.add_user(data, vec![]).await.map_err(|err| {
            error!("注册邮箱用户失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("注册邮箱用户失败")
        })?;
        Ok(result)
    }
}
