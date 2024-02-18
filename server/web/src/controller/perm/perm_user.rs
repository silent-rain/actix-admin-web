//! 用户管理

use crate::{
    dto::perm::perm_user::{AddUserReq, DeleteUserReq, GetUserInfoReq, GetUserListReq},
    inject::Provider,
    service::perm::perm_user::PermUserService,
};

use code::Error;
use response::Response;

use actix_web::{web, Responder};
use validator::Validate;

/// 控制器
pub struct Controller;

impl Controller {
    /// 用户列表查询
    pub async fn list(
        provider: web::Data<Provider>,
        req: web::Query<GetUserListReq>,
    ) -> impl Responder {
        let perm_user_service: PermUserService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data_list(results, total)
    }

    /// 用户详情查询
    pub async fn info(
        provider: web::Data<Provider>,
        params: web::Query<GetUserInfoReq>,
    ) -> impl Responder {
        let perm_user_service: PermUserService = provider.provide();
        let resp = perm_user_service.info(params.id).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };
        let result = match result {
            Some(v) => v,
            None => return Response::build().code(Error::DbQueryEmptyError),
        };

        Response::build().data(result)
    }

    /// 添加用户信息
    pub async fn add(provider: web::Data<Provider>, data: web::Json<AddUserReq>) -> impl Responder {
        let data = data.into_inner();
        if let Err(e) = data.validate() {
            return Response::build()
                .code(Error::InvalidParameterError)
                .msg(&e.to_string());
        }

        let perm_user_service: PermUserService = provider.provide();
        let resp = perm_user_service.add(data).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data(result)
    }

    /// 删除用户
    pub async fn delete(
        provider: web::Data<Provider>,
        params: web::Query<DeleteUserReq>,
    ) -> impl Responder {
        let perm_user_service: PermUserService = provider.provide();
        let resp = perm_user_service.delete(params.id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().msg("删除成功")
    }
}
