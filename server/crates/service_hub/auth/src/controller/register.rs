//! 注册

use crate::{dto::register::RegisterReq, enums::RegisterType, RegisterService};

use actix_validator::Json;
use code::Error;
use inject::AInjectProvider;
use response::Response;

use actix_web::{web::Data, Responder};
use tracing::error;

/// 控制器
pub struct RegisterController;

impl RegisterController {
    /// 注册用户
    pub async fn register(
        provider: Data<AInjectProvider>,
        data: Json<RegisterReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        match data.register_type {
            RegisterType::Phone => {
                if data.phone.is_none() {
                    error!("请输入手机号码");
                    return Response::err(
                        Error::InvalidParameterError
                            .into_msg()
                            .with_msg("请输入手机号码"),
                    );
                }
            }
            RegisterType::Email => {
                if data.email.is_none() {
                    error!("请输入邮箱");
                    return Response::err(
                        Error::InvalidParameterError
                            .into_msg()
                            .with_msg("请输入邮箱"),
                    );
                }
            }
        }

        let register_service: RegisterService = provider.provide();
        let result = register_service.register(data).await;
        match result {
            Ok(_v) => Response::ok().with_msg("注册成功"),
            Err(err) => Response::err(err),
        }
    }
}
