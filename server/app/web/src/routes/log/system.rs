//! 系统日志

use crate::state::AppState;
use code::Error;
use dto::log::system::{DeleteSystemLogReq, SystemLogInfoReq, SystemLogListReq};
use entity::log_system::Model;
use response::Response;

use actix_web::{web, Responder, Scope};
use service::log::system;

/// 系统日志管理
pub fn register() -> Scope {
    web::scope("/system_log")
        .route("/list", web::get().to(Routes::list))
        .route("/info", web::get().to(Routes::info))
        .route("/add", web::post().to(Routes::add))
        .route("/delete", web::delete().to(Routes::delete))
}

/// 路由层
pub struct Routes;

impl Routes {
    /// 系统日志列表查询
    pub async fn list(
        state: web::Data<AppState>,
        req: web::Query<SystemLogListReq>,
    ) -> impl Responder {
        let resp = system::Service::new(&state.db).list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data_list(results, total)
    }

    /// 系统日志详情查询
    pub async fn info(
        state: web::Data<AppState>,
        params: web::Query<SystemLogInfoReq>,
    ) -> impl Responder {
        let resp = system::Service::new(&state.db).info(params.id).await;
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

    /// 添加系统日志
    pub async fn add(state: web::Data<AppState>, data: web::Json<Model>) -> impl Responder {
        let data = data.into_inner();
        let resp = system::Service::new(&state.db).add(data).await;
        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data(result)
    }

    /// 删除系统日志
    pub async fn delete(
        state: web::Data<AppState>,
        params: web::Query<DeleteSystemLogReq>,
    ) -> impl Responder {
        let resp = system::Service::new(&state.db).delete(params.id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().msg("删除成功")
    }
}
