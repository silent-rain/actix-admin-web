//! 登陆

use crate::{
    app::{
        auth::dto::login::{LoginReq, LoginRsp},
        perm::UserService,
        system::{CaptchaService, UserLoginService},
    },
    inject::AProvider,
};

use actix_validator::Json;
use chrono::Local;
use code::Error;
use entity::{perm_user, sys_user_login};
use jwt::encode_token;
use response::Response;

use actix_web::{web::Data, HttpRequest, Responder};
use sea_orm::Set;

/// 控制器
pub struct LoginController;

impl LoginController {
    /// 登陆
    pub async fn login(
        req: HttpRequest,
        provider: Data<AProvider>,
        data: Json<LoginReq>,
    ) -> impl Responder {
        let data = data.into_inner();

        // 检测验证码
        if let Err(err) = Self::check_captcha(
            provider.clone(),
            data.captcha_id.clone(),
            data.captcha.clone(),
        )
        .await
        {
            return err;
        }

        // 检测手机号码或邮件
        let user = match Self::get_username(provider.clone(), data.clone()).await {
            Ok(v) => v,
            Err(err) => return err,
        };
        // 检测密码
        if user.password != data.password {
            return Response::code(Error::LoginPasswordError);
        }

        // 生成Token
        let token = match encode_token(user.id, user.username.clone().map_or("".to_owned(), |v| v))
        {
            Ok(v) => v,
            Err(err) => return Response::code(Error::TokenEncode(err.to_string())),
        };

        // 添加登陆日志
        if let Err(err) = Self::add_login_log(req, provider.clone(), user.clone()).await {
            return err;
        };

        // 返回Token
        Response::ok().data(LoginRsp {
            user_id: user.id,
            token,
        })
    }

    /// 检测验证码
    async fn check_captcha(
        provider: Data<AProvider>,
        captcha_id: String,
        captcha: String,
    ) -> Result<(), Response> {
        let captcha_service: CaptchaService = provider.provide();
        let captcha_m = captcha_service.info_by_captcha_id(captcha_id).await;
        let captcha_m = match captcha_m {
            Ok(v) => v,
            Err(err) => match err {
                Error::DbQueryEmptyError => return Err(Response::code(Error::CaptchaNotExist)),
                _ => return Err(Response::code(err).msg("验证服务请求失败")),
            },
        };

        if captcha_m.captcha.to_uppercase() != captcha.to_uppercase() {
            return Err(Response::code(Error::CaptchaInvalid));
        }

        let max_time = captcha_m.created_at.and_utc().timestamp() + captcha_m.expire as i64;
        let now = Local::now().timestamp();
        if now > max_time {
            return Err(Response::code(Error::CaptchaExpire));
        }

        Ok(())
    }

    /// 获取用户信息
    async fn get_username(
        provider: Data<AProvider>,
        data: LoginReq,
    ) -> Result<perm_user::Model, Response> {
        let user_service: UserService = provider.provide();
        let result = user_service.info_by_username(data.username.clone()).await;
        let result = match result {
            Ok(v) => v,
            Err(err) => return Err(Response::code(err)),
        };
        match result {
            Some(v) => Ok(v),
            None => Err(Response::code(code::Error::DbQueryEmptyError).msg("该用户不存在")),
        }
    }

    /// 添加登陆日志
    async fn add_login_log(
        req: HttpRequest,
        provider: Data<AProvider>,
        user: perm_user::Model,
    ) -> Result<(), Response> {
        let user_login_service: UserLoginService = provider.provide();
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

        if let Err(err) = user_login_service.add(data).await {
            return Err(
                Response::code(code::Error::DBAddError(err.to_string())).msg("添加登陆日志失败")
            );
        }
        Ok(())
    }
}
