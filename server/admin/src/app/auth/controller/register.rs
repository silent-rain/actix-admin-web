//! 注册

use crate::{
    app::auth::{
        dto::register::{RegisterReq, RegisterType},
        RegisterService,
    },
    inject::AProvider,
};

use actix_validator::Json;
use code::Error;
use response::Response;

use actix_web::{web::Data, Responder};

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
            }
            RegisterType::Email => {
                if data.email.is_none() {
                    return Response::code(Error::InvalidParameterError("请输入邮箱".to_owned()));
                }
            }
        }

        let register_service: RegisterService = provider.provide();
        let result = register_service.register(data).await;
        match result {
            Ok(_v) => Response::ok().msg("注册成功"),
            Err(err) => Response::code(err),
        }
    }
}
