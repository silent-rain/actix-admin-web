//! 系统日志

use crate::{
    app::log::{dto::system::LogSystemListReq, service::system::LogSystemService},
    inject::AProvider,
};

use entity::log::system;
use response::Response;

use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};

/// 控制器
pub struct LogSystemController;

impl LogSystemController {
    /// 获取系统日志列表
    pub async fn list(provider: Data<AProvider>, req: Query<LogSystemListReq>) -> impl Responder {
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data_list(results, total)
    }

    /// 获取系统日志的详细信息
    pub async fn info(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.info(*id).await;
        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 添加新的系统日志
    pub async fn add(provider: Data<AProvider>, data: Json<system::Model>) -> impl Responder {
        let data = data.into_inner();
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.add(data).await;
        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 删除系统日志
    pub async fn delete(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.delete(*id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("删除成功")
    }
}
