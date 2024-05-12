//! WEB日志管理

use crate::{
    dto::web_log::{AddWebLogInfoReq, GetWebLogListReq},
    service::web_log::WebLogService,
};

use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};

/// 控制器
pub struct WebLogController;

impl WebLogController {
    /// 获取WEB日志列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetWebLogListReq>,
    ) -> impl Responder {
        let log_web_service: WebLogService = provider.provide();
        let resp = log_web_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取WEB日志信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let log_web_service: WebLogService = provider.provide();
        let resp = log_web_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加WEB日志
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddWebLogInfoReq>,
    ) -> impl Responder {
        let log_web_service: WebLogService = provider.provide();
        let resp = log_web_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
