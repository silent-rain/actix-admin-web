//! 登陆

use std::sync::Arc;

use crate::{
    common::captcha::check_captcha,
    dto::login::{BrowserInfo, LoginReq, LoginRsp},
};

use database::PoolTrait;
use log::UserLoginDao;
use system::ImageCaptchaDao;
use user::{EmailDao, PhoneDao, UserBaseDao};

use code::{Error, ErrorMsg};
use entity::{log_user_login, user::user_base};
use jwt::encode_token;

use nject::injectable;
use sea_orm::Set;
use tracing::error;
use utils::browser::parse_user_agent_async;

/// 服务层
#[injectable]
pub struct LoginService {
    db: Arc<dyn PoolTrait>,
    user_dao: UserBaseDao,
    email_dao: EmailDao,
    phone_dao: PhoneDao,
    captcha_dao: ImageCaptchaDao,
}

impl LoginService {
    /// 登陆
    pub async fn login(
        &self,
        browser_info: BrowserInfo,
        data: LoginReq,
    ) -> Result<LoginRsp, ErrorMsg> {
        // 检测验证码
        check_captcha(
            &self.captcha_dao,
            data.captcha_id.clone(),
            data.captcha.clone(),
        )
        .await?;

        // 检测手机号码或邮件用户是否存在
        let user = self.get_user(data.clone()).await?;
        // 检查用户是否被禁用
        if user.status == user_base::enums::Status::Disabled as i8 {
            // 添加登陆日志
            self.add_login_log(
                user.clone(),
                browser_info,
                "".to_owned(),
                Some("用户已被禁用".to_owned()),
                log_user_login::enums::Status::Failed,
            );
            error!("用户已被禁用");
            return Err(Error::LoginUserDisableError
                .into_msg()
                .with_msg("用户已被禁用"));
        }
        // 检测密码
        if user.password != data.password {
            // 添加失败登陆日志
            self.add_login_log(
                user.clone(),
                browser_info,
                "".to_owned(),
                Some("账号或密码错误".to_owned()),
                log_user_login::enums::Status::Failed,
            );
            error!("账号或密码错误");
            return Err(Error::LoginPasswordError
                .into_msg()
                .with_msg("账号或密码错误"));
        }

        // 生成Token
        let token = encode_token(user.id, user.username.clone()).map_err(|err| {
            error!("生成密匙失败, err: {}", err);
            Error::TokenEncode.into_msg().with_msg("生成密匙失败")
        })?;

        // 添加登陆日志
        self.add_login_log(
            user.clone(),
            browser_info,
            token.clone(),
            None,
            log_user_login::enums::Status::Success,
        );

        // 返回Token
        Ok(LoginRsp {
            user_id: user.id,
            token,
        })
    }

    /// 获取用户信息
    async fn get_user(&self, data: LoginReq) -> Result<user_base::Model, ErrorMsg> {
        let user_id = match data.user_type {
            user_base::enums::UserType::Phone => self.get_user_phone(data).await?,
            user_base::enums::UserType::Email => self.get_user_email(data).await?,
        };

        // 查询用户
        let result = self
            .user_dao
            .info(user_id)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("该用户不存在")
            })?;

        Ok(result)
    }

    /// 获取用户手机号
    async fn get_user_phone(&self, data: LoginReq) -> Result<i32, ErrorMsg> {
        let phone = match data.phone.clone() {
            Some(v) => v,
            None => {
                return Err(code::Error::InvalidParameter
                    .into_msg()
                    .with_msg("请求参数错误, phone 不能为空"))
            }
        };

        let user = self
            .phone_dao
            .info_by_phone(phone)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户手机号不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("该用户手机号不存在")
            })?;

        Ok(user.user_id)
    }

    /// 获取用户邮箱
    async fn get_user_email(&self, data: LoginReq) -> Result<i32, ErrorMsg> {
        let email = match data.email.clone() {
            Some(v) => v,
            None => {
                return Err(code::Error::InvalidParameter
                    .into_msg()
                    .with_msg("请求参数错误, phone 不能为空"))
            }
        };

        let user = self
            .email_dao
            .info_by_email(email)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户邮箱不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("该用户邮箱不存在")
            })?;

        Ok(user.user_id)
    }

    /// 添加登陆日志
    fn add_login_log(
        &self,
        user: user_base::Model,
        browser_info: BrowserInfo,
        token: String,
        desc: Option<String>,
        status: log_user_login::enums::Status,
    ) {
        let db = self.db.clone();

        actix_web::rt::spawn(async move {
            let (device, system, browser) =
                match parse_user_agent_async(browser_info.user_agent.clone()).await {
                    Ok(v) => v,
                    Err(err) => {
                        error!("User-Agent解析错误, err: {:#?}", err);
                        return;
                    }
                };

            let data = log_user_login::ActiveModel {
                user_id: Set(user.id),
                username: Set(user.username),
                token: Set(token),
                remote_addr: Set(browser_info.remote_addr),
                user_agent: Set(browser_info.user_agent),
                status: Set(status as i8),
                device: Set(Some(device)),
                system: Set(Some(system)),
                browser: Set(Some(browser)),
                desc: Set(desc),
                ..Default::default()
            };

            let user_login_dao = UserLoginDao::new(db);
            let result = user_login_dao.add(data).await.map_err(|err| {
                error!("添加登陆日志失败, err: {:#?}", err);
                code::Error::DbAddError
                    .into_msg()
                    .with_msg("添加登陆日志失败")
            });
            if let Err(err) = result {
                error!("添加登陆日志失败, err: {:#?}", err);
            }
        });
    }
}
