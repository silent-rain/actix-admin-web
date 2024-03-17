//! 用户管理

use crate::{
    dto::perm::perm_user::{AddUserReq, DeleteUserReq, GetUserInfoReq, GetUserListReq},
    inject::AProvider,
    service::perm::perm_user::PermUserService,
};

use actix_validator::{Json, Query};
use code::Error;
use response::Response;

use actix_web::{web::Data, Responder};
// use validator::Validate;

/// 控制器
pub struct Controller;

impl Controller {
    /// 用户列表查询
    pub async fn list(provider: Data<AProvider>, req: Query<GetUserListReq>) -> impl Responder {
        let perm_user_service: PermUserService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data_list(results, total)
    }

    /// 用户详情查询
    pub async fn info(provider: Data<AProvider>, params: Query<GetUserInfoReq>) -> impl Responder {
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

    /// 添加用户信息
    pub async fn add(provider: Data<AProvider>, data: Json<AddUserReq>) -> impl Responder {
        let perm_user_service: PermUserService = provider.provide();
        let resp = perm_user_service.add(data.into_inner()).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data(result)
    }

    /// 删除用户
    pub async fn delete(provider: Data<AProvider>, params: Query<DeleteUserReq>) -> impl Responder {
        let perm_user_service: PermUserService = provider.provide();
        let resp = perm_user_service.delete(params.id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().msg("删除成功")
    }
}
