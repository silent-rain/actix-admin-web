//! 登陆

use crate::{
    auth::{
        common::captcha::check_captcha,
        dto::login::{BrowserInfo, LoginReq, LoginRsp},
    },
    log::{
        enums::{UserLoginDisabledStatus, UserLoginStatus},
        UserLoginDao,
    },
    perm::{enums::UserStatus, UserDao},
    system::CaptchaDao,
};

use code::{Error, ErrorMsg};
use entity::{log_user_login, perm_user};
use jwt::encode_token;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct LoginService<'a> {
    user_dao: UserDao<'a>,
    user_login_dao: UserLoginDao<'a>,
    captcha_dao: CaptchaDao<'a>,
}

impl<'a> LoginService<'a> {
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
        let user = self.get_username(data.clone()).await?;
        // 检查用户是否被禁用
        if user.status == UserStatus::Disabled as i8 {
            // 添加失败登陆日志
            self.add_login_log(
                browser_info,
                user.clone(),
                "".to_owned(),
                UserLoginStatus::Failed,
            )
            .await?;
            error!("用户已被禁用");
            return Err(Error::LoginUserDisableError
                .into_msg()
                .with_msg("用户已被禁用"));
        }
        // 检测密码
        if user.password != data.password {
            // 添加失败登陆日志
            self.add_login_log(
                browser_info,
                user.clone(),
                "".to_owned(),
                UserLoginStatus::Failed,
            )
            .await?;
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
            browser_info,
            user.clone(),
            token.clone(),
            UserLoginStatus::Success,
        )
        .await?;

        // 返回Token
        Ok(LoginRsp {
            user_id: user.id,
            token,
        })
    }

    /// 获取用户信息
    async fn get_username(&self, data: LoginReq) -> Result<perm_user::Model, ErrorMsg> {
        let result = self
            .user_dao
            .info_by_username(data.username.clone())
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

    /// 添加登陆日志
    async fn add_login_log(
        &self,
        browser_info: BrowserInfo,
        user: perm_user::Model,
        token: String,
        status: UserLoginStatus,
    ) -> Result<log_user_login::Model, ErrorMsg> {
        let data = log_user_login::ActiveModel {
            user_id: Set(user.id),
            username: Set(user.username),
            token: Set(token),
            remote_addr: Set(browser_info.remote_addr),
            user_agent: Set(browser_info.user_agent),
            status: Set(status as i8),
            disabled: Set(UserLoginDisabledStatus::Enabled as i8),
            ..Default::default()
        };

        let result = self.user_login_dao.add(data).await.map_err(|err| {
            error!("添加登陆日志失败, err: {:#?}", err);
            code::Error::DbAddError
                .into_msg()
                .with_msg("添加登陆日志失败")
        })?;

        Ok(result)
    }
}
