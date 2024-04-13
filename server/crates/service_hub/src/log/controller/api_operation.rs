//! API操作日志

use crate::{
    inject::AInjectProvider,
    log::{
        dto::api_operation::{AddApiOperationReq, GetApiOperationListReq},
        service::api_operation::ApiOperationService,
    },
};

use response::Response;

use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};

/// 控制器
pub struct ApiOperationController;

impl ApiOperationController {
    /// 获取API操作日志列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetApiOperationListReq>,
    ) -> impl Responder {
        let system_service: ApiOperationService = provider.provide();
        let resp = system_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取API操作日志的详细信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let system_service: ApiOperationService = provider.provide();
        let resp = system_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加API操作日志
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddApiOperationReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let system_service: ApiOperationService = provider.provide();
        let resp = system_service.add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除API操作日志
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let system_service: ApiOperationService = provider.provide();
        let resp = system_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
