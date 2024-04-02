//! 用户管理

use crate::{
    app::perm::{
        dto::user::{AddUserReq, DeleteUserReq, GetUserInfoReq, GetUserListReq},
        service::user::UserService,
    },
    inject::AProvider,
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct UserController;

impl UserController {
    /// 获取用户列表
    pub async fn list(provider: Data<AProvider>, req: Query<GetUserListReq>) -> impl Responder {
        let perm_user_service: UserService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data_list(results, total)
    }

    /// 获取用户信息
    pub async fn info(provider: Data<AProvider>, params: Query<GetUserInfoReq>) -> impl Responder {
        let perm_user_service: UserService = provider.provide();
        let resp = perm_user_service.info(params.id).await;

        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 添加用户
    pub async fn add(provider: Data<AProvider>, data: Json<AddUserReq>) -> impl Responder {
        let perm_user_service: UserService = provider.provide();

        let resp = perm_user_service.add(data.into_inner()).await;
        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 删除用户
    pub async fn delete(provider: Data<AProvider>, params: Query<DeleteUserReq>) -> impl Responder {
        let perm_user_service: UserService = provider.provide();
        let resp = perm_user_service.delete(params.id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("删除成功")
    }
}
