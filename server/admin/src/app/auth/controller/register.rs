//! 注册

use crate::{
    app::auth::{
        dto::register::{EmailRegisterReq, PhoneRegisterReq, RegisterReq, RegisterType},
        RegisterService,
    },
    app::perm::UserService,
    app::system::CaptchaService,
    inject::AProvider,
};

use actix_validator::Json;
use code::Error;
use response::Response;
use utils::crypto::make_md5;
use utils::json::struct_to_struct;

use actix_web::{web::Data, Responder};
use chrono::Local;

/// 控制器
pub struct RegisterController;

impl RegisterController {
    /// 注册用户
    pub async fn register(provider: Data<AProvider>, data: Json<RegisterReq>) -> impl Responder {
        let data = data.into_inner();
        match data.register_type {
            RegisterType::Phone => {
                if data.phone.is_none() {
                    return Response::code(Error::InvalidParameterError(
                        "请输入手机号码".to_owned(),
                    ));
                }
                let data: PhoneRegisterReq = match struct_to_struct(&data) {
                    Ok(v) => v,
                    Err(err) => return Response::code(err),
                };
                Self::phone_register(provider, Json(data)).await
            }
            RegisterType::Email => {
                if data.email.is_none() {
                    return Response::code(Error::InvalidParameterError("请输入邮箱".to_owned()));
                }
                let data: EmailRegisterReq = match struct_to_struct(&data) {
                    Ok(v) => v,
                    Err(err) => return Response::code(err),
                };
                Self::email_register(provider, Json(data)).await
            }
        }
    }

    /// 注册手机用户
    pub async fn phone_register(
        provider: Data<AProvider>,
        data: Json<PhoneRegisterReq>,
    ) -> Response {
        let mut data = data.into_inner();

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

        // TODO 检测手机验证码, 待接入第三方服务

        // 检测是否已注册用户
        let user_service: UserService = provider.provide();
        let user = user_service.info_by_phone(data.phone.clone()).await;
        let user = match user {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };
        if user.is_some() {
            return Response::code(code::Error::DBDataExistError).msg("该手机号码已注册");
        }

        // 密码加密
        data.password = make_md5(&data.password);

        let register_service: RegisterService = provider.provide();
        let resp = register_service.add_phone_user(data).await;
        if let Err(err) = resp.map_err(Response::code) {
            return err;
        }

        Response::ok().msg("注册成功")
    }

    /// 注册邮件用户
    pub async fn email_register(
        provider: Data<AProvider>,
        data: Json<EmailRegisterReq>,
    ) -> Response {
        let mut data = data.into_inner();

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

        // 检测是否已注册邮件
        let user_service: UserService = provider.provide();
        let user = user_service.info_by_email(data.email.clone()).await;
        let user = match user {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };
        if user.is_some() {
            return Response::code(code::Error::DBDataExistError).msg("该邮箱已注册");
        }

        // 密码加密
        data.password = make_md5(&data.password);

        let register_service: RegisterService = provider.provide();
        let resp = register_service.add_email_user(data).await;
        if let Err(err) = resp.map_err(Response::code) {
            return err;
        }

        // TODO 邮箱验证, 发送链接点击后确认

        Response::ok().msg("注册成功, 请验证")
    }

    /// 检测验证码
    async fn check_captcha(
        provider: Data<AProvider>,
        captcha_id: String,
        captcha: String,
    ) -> Result<(), Response> {
        let captcha_service: CaptchaService = provider.provide();
        let captcha_m = captcha_service.info(captcha_id).await;
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
}
