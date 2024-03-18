//! 登陆

use crate::{
    dto::perm::perm_user::GetUserInfoReq, inject::AProvider,
    service::perm::perm_user::PermUserService,
};

use actix_validator::Query;
use code::Error;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct Controller;

impl Controller {
    /// 登陆
    pub async fn login(provider: Data<AProvider>, params: Query<GetUserInfoReq>) -> impl Responder {
        let perm_user_service: PermUserService = provider.provide();
        let resp = perm_user_service.info(params.id).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };
        let result = match result {
            Some(v) => v,
            None => return Response::code(Error::DbQueryEmptyError),
        };

        Response::ok().data(result)
    }
}
