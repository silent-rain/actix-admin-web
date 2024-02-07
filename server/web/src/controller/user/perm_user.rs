//! 用户管理

use crate::{
    dto::user::perm_user::{AddUserReq, DeleteUserReq, UserInfoReq, UserListReq},
    service::user::perm_user::PermUserService,
    state::AppState,
};

use code::Error;
use response::Response;

use actix_web::{web, Responder};
use validator::Validate;

/// 控制器
pub struct Controller;

impl Controller {
    /// 用户列表查询
    pub async fn list(state: web::Data<AppState>, req: web::Query<UserListReq>) -> impl Responder {
        let resp = PermUserService::new(&state.db).list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data_list(results, total)
    }

    /// 用户详情查询
    pub async fn info(
        state: web::Data<AppState>,
        params: web::Query<UserInfoReq>,
    ) -> impl Responder {
        let resp = PermUserService::new(&state.db).info(params.id).await;

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
    pub async fn add(state: web::Data<AppState>, data: web::Json<AddUserReq>) -> impl Responder {
        let data = data.into_inner();
        if let Err(e) = data.validate() {
            return Response::build()
                .code(Error::InvalidParameterError)
                .msg(&e.to_string());
        }

        let resp = PermUserService::new(&state.db).add(data).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data(result)
    }

    /// 删除用户
    pub async fn delete(
        state: web::Data<AppState>,
        params: web::Query<DeleteUserReq>,
    ) -> impl Responder {
        let resp = PermUserService::new(&state.db).delete(params.id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().msg("删除成功")
    }
}
