//! 注册

use crate::{
    dto::perm::perm_user::{AddUserReq, DeleteUserReq, GetUserInfoReq, GetUserListReq},
    inject::AProvider,
    service::perm::perm_user::PermUserService,
};

use actix_validator::{Json, Query};
use code::Error;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct Controller;

impl Controller {
    /// 注册用户
    pub async fn register(provider: Data<AProvider>, data: Json<AddUserReq>) -> impl Responder {
        let perm_user_service: PermUserService = provider.provide();
        let resp = perm_user_service.add(data.into_inner()).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data(result)
    }
}
