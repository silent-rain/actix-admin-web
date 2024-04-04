//! 系统日志

use crate::{
    app::log::{dto::system::LogSystemListReq, service::system::LogSystemService},
    inject::AProvider,
};

use entity::log_system;
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
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::code(err),
        }
    }

    /// 获取系统日志的详细信息
    pub async fn info(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::code(err),
        }
    }

    /// 添加新的系统日志
    pub async fn add(provider: Data<AProvider>, data: Json<log_system::Model>) -> impl Responder {
        let data = data.into_inner();
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 删除系统日志
    pub async fn delete(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let log_system_service: LogSystemService = provider.provide();
        let resp = log_system_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }
}
