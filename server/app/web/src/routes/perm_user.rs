//! 用户管理

use crate::state::AppState;
use code::Error;
use dto::perm_user::{AddUserReq, UserInfoReq, UserListReq};
use response::Response;
use service::user;

use actix_web::{web, Responder};
use validator::Validate;

/// 路由层
pub struct Routes;

impl Routes {
    /// 用户列表查询
    pub async fn list(state: web::Data<AppState>, req: web::Query<UserListReq>) -> impl Responder {
        let resp = user::Service::new(&state.db).list(req.into_inner()).await;
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
        let resp = user::Service::new(&state.db).info(params.id).await;

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

        let resp = user::Service::new(&state.db).add(data).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data(result)
    }
}
