//! 登陆

use crate::{
    app::perm::{dto::user::GetUserInfoReq, UserService},
    inject::AProvider,
};

use actix_validator::Query;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct LoginController;

impl LoginController {
    /// 登陆
    pub async fn login(provider: Data<AProvider>, params: Query<GetUserInfoReq>) -> impl Responder {
        let perm_user_service: UserService = provider.provide();
        let resp = perm_user_service.info(params.id).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data(result)
    }
}
