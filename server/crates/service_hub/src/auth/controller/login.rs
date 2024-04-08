//! 登陆

use crate::{
    auth::{dto::login::LoginReq, LoginService},
    inject::AInjectProvider,
};

use actix_validator::Json;
use response::Response;

use actix_web::{web::Data, HttpRequest, Responder};

/// 控制器
pub struct LoginController;

impl LoginController {
    /// 登陆
    pub async fn login(
        req: HttpRequest,
        provider: Data<AInjectProvider>,
        data: Json<LoginReq>,
    ) -> impl Responder {
        let login_service: LoginService = provider.provide();
        let result = login_service.login(req, data.into_inner()).await;
        match result {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }
}
