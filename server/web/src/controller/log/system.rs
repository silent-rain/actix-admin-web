//! 系统日志
use std::sync::Arc;

use crate::dto::log::log_system::{DeleteLogSystemReq, LogSystemInfoReq, LogSystemListReq};
use crate::inject::Provider;
use crate::service::log::system::LogSystemService;

use code::Error;
use entity::log::system::Model;
use response::Response;

use actix_web::{
    web::{Data, Json, Query},
    Responder,
};

/// 控制器
pub struct Controller;

impl Controller {
    /// 系统日志列表查询
    pub async fn list(
        provider: Data<Arc<Provider>>,
        req: Query<LogSystemListReq>,
    ) -> impl Responder {
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data_list(results, total)
    }

    /// 系统日志详情查询
    pub async fn info(
        provider: Data<Arc<Provider>>,
        params: Query<LogSystemInfoReq>,
    ) -> impl Responder {
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.info(params.id).await;
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
    pub async fn add(provider: Data<Arc<Provider>>, data: Json<Model>) -> impl Responder {
        let data = data.into_inner();
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.add(data).await;
        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data(result)
    }

    /// 删除系统日志
    pub async fn delete(
        provider: Data<Arc<Provider>>,
        params: Query<DeleteLogSystemReq>,
    ) -> impl Responder {
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.delete(params.id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().msg("删除成功")
    }
}
