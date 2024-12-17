//! 系统日志

use crate::{dto::system::GetSystemListReq, service::system::SystemService};

use entity::log::log_system;
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};

/// 控制器
pub struct SystemController;

impl SystemController {
    /// 获取系统日志列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetSystemListReq>,
    ) -> impl Responder {
        let system_service: SystemService = provider.provide();
        let resp = system_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取系统日志的详细信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let system_service: SystemService = provider.provide();
        let resp = system_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加新的系统日志
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<log_system::Model>,
    ) -> impl Responder {
        let data = data.into_inner();
        let system_service: SystemService = provider.provide();
        let resp = system_service.add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除系统日志
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let system_service: SystemService = provider.provide();
        let resp = system_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
