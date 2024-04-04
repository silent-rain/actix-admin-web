//! 登陆

use crate::app::{
    auth::{
        common::captcha::check_captcha,
        dto::login::{LoginReq, LoginRsp},
    },
    perm::UserDao,
    system::{CaptchaDao, UserLoginDao},
};

use actix_web::HttpRequest;
use code::Error;
use entity::{perm_user, sys_user_login};
use jwt::encode_token;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务
#[injectable]
pub struct LoginService<'a> {
    user_dao: UserDao<'a>,
    user_login_dao: UserLoginDao<'a>,
    captcha_dao: CaptchaDao<'a>,
}

impl<'a> LoginService<'a> {
    /// 登陆
    pub async fn login(&self, req: HttpRequest, data: LoginReq) -> Result<LoginRsp, Error> {
        // 检测验证码
        check_captcha(
            &self.captcha_dao,
            data.captcha_id.clone(),
            data.captcha.clone(),
        )
        .await?;

        // 检测手机号码或邮件用户是否存在
        let user = self.get_username(data.clone()).await?;
        // 检测密码
        if user.password != data.password {
            error!("账号或密码错误");
            return Err(Error::LoginPasswordError);
        }

        // 生成Token
        let token = encode_token(user.id, user.username.clone().map_or("".to_owned(), |v| v))
            .map_err(|err| {
                error!("获取密匙异常, err: {}", err);
                Error::TokenEncode
            })?;

        // 添加登陆日志
        self.add_login_log(req, user.clone()).await?;

        // 返回Token
        Ok(LoginRsp {
            user_id: user.id,
            token,
        })
    }

    /// 获取用户信息
    async fn get_username(&self, data: LoginReq) -> Result<perm_user::Model, Error> {
        let result = self
            .user_dao
            .info_by_username(data.username.clone())
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError
            })?
            .ok_or_else(|| {
                error!("该用户不存在");
                Error::DbQueryEmptyError
            })?;

        Ok(result)
    }

    /// 添加登陆日志
    async fn add_login_log(
        &self,
        req: HttpRequest,
        user: perm_user::Model,
    ) -> Result<sys_user_login::Model, Error> {
        let username = user.username.map_or("".to_owned(), |v| v);
        // Get the remote address from the request
        // let remote_addr = req
        //     .connection_info()
        //     .remote_addr()
        //     .map_or("".to_owned(), |addr| addr.to_string());
        let remote_addr = req
            .peer_addr()
            .map_or("".to_owned(), |addr| addr.to_string());
        // Get the user agent from the request headers
        let user_agent = req
            .headers()
            .get("User-Agent")
            .map_or("".to_owned(), |ua| ua.to_str().unwrap_or("").to_owned());

        let data = sys_user_login::ActiveModel {
            user_id: Set(user.id),
            username: Set(username),
            remote_addr: Set(remote_addr),
            user_agent: Set(user_agent),
            status: Set(1),
            ..Default::default()
        };

        let result = self.user_login_dao.add(data).await.map_err(|err| {
            error!("添加登陆日志失败, err: {:#?}", err);
            code::Error::DbAddError
        })?;

        Ok(result)
    }
}
