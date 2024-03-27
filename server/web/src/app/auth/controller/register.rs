//! 注册

use crate::{
    app::auth::{
        dto::register::{EmailRegisterReq, PhoneRegisterReq, RegisterReq, RegisterType},
        service::register::RegisterService,
    },
    inject::AProvider,
};

use actix_validator::Json;
use code::Error;
use response::Response;
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

        Response::ok().data(result)
    }

    /// 注册手机用户
    pub async fn phone_register(
        provider: Data<AProvider>,
        data: Json<PhoneRegisterReq>,
    ) -> impl Responder {
        let register_service: RegisterService = provider.provide();

        // TODO
        // 用户是否已注册检测
        // 手机验证码验证
        // 密码加密

        let resp = register_service
            .add_phone_user(data.into_inner())
            .await
            .map_err(Response::code);

        let result = match resp {
            Ok(v) => v,
            Err(e) => return e,
        };

        Response::ok().data(result)
    }

    /// 注册邮件用户
    pub async fn email_register(
        provider: Data<AProvider>,
        data: Json<EmailRegisterReq>,
    ) -> impl Responder {
        let register_service: RegisterService = provider.provide();

        // TODO
        // 用户是否已注册检测
        // 邮箱验证, 发送链接点击后确认
        // 密码加密

        let resp = register_service.add_email_user(data.into_inner()).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data(result)
    }
}
