//! 注册

use crate::{
    app::auth::{
        dto::register::{EmailRegisterReq, PhoneRegisterReq, RegisterReq, RegisterType},
        RegisterService,
    },
    app::perm::UserService,
    inject::AProvider,
};

use actix_validator::Json;
use code::Error;
use response::Response;
use utils::crypto::make_md5;
use utils::json::struct_to_struct;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct RegisterController;

impl RegisterController {
    /// 注册用户
    pub async fn register(provider: Data<AProvider>, data: Json<RegisterReq>) -> impl Responder {
        let register_service: RegisterService = provider.provide();
        let data = data.into_inner();
        let resp = match data.register_type {
            RegisterType::Phone => {
                if data.phone.is_none() {
                    return Error::InvalidParameterError("请输入手机号码".to_string()).into();
                }
                let data: PhoneRegisterReq = match struct_to_struct(&data) {
                    Ok(v) => v,
                    Err(err) => return Response::code(err),
                };
                register_service.add_phone_user(data).await
            }
            RegisterType::Email => {
                if data.email.is_none() {
                    return Error::InvalidParameterError("请输入邮箱".to_string()).into();
                }
                let data: EmailRegisterReq = match struct_to_struct(&data) {
                    Ok(v) => v,
                    Err(err) => return Response::code(err),
                };
                register_service.add_email_user(data).await
            }
        };

        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("注册成功")
    }

    /// 注册手机用户
    pub async fn phone_register(
        provider: Data<AProvider>,
        data: Json<PhoneRegisterReq>,
    ) -> impl Responder {
        let mut data = data.into_inner();

        // TODO 验证码检测
        // TODO 手机验证码检测

        // 用户是否已注册检测
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
    ) -> impl Responder {
        let mut data = data.into_inner();

        // TODO
        // 验证码检测
        // 用户是否已注册检测

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
}
