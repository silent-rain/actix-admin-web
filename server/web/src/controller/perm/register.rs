//! 注册

use crate::{
    dto::perm::register::{EmailRegisterReq, PhoneRegisterReq},
    inject::AProvider,
    service::perm::register::RegisterService,
};

use actix_validator::Json;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct Controller;

impl Controller {
    /// 注册手机用户
    pub async fn phone_register(
        provider: Data<AProvider>,
        data: Json<PhoneRegisterReq>,
    ) -> impl Responder {
        let register_service: RegisterService = provider.provide();

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

        let resp = register_service.add_email_user(data.into_inner()).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data(result)
    }
}
