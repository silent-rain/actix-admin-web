//! OpenApi接口管理

use crate::{
    dto::openapi::{AddOpenapiReq, GetOpenapiListReq, UpdateOpenapiReq, UpdateOpenapiStatusReq},
    service::openapi::OpenapiService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct OpenapiController;

impl OpenapiController {
    /// 获取OpenApi接口列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetOpenapiListReq>,
    ) -> impl Responder {
        let openapi_service: OpenapiService = provider.provide();
        let resp = openapi_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取OpenApi接口树列表
    pub async fn tree(provider: Data<AInjectProvider>) -> impl Responder {
        let openapi_service: OpenapiService = provider.provide();
        let resp = openapi_service.tree().await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取OpenApi接口信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let openapi_service: OpenapiService = provider.provide();
        let resp = openapi_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加OpenApi接口
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddOpenapiReq>) -> impl Responder {
        let openapi_service: OpenapiService = provider.provide();
        let resp = openapi_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新OpenApi接口
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateOpenapiReq>,
    ) -> impl Responder {
        let openapi_service: OpenapiService = provider.provide();
        let resp = openapi_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新OpenApi接口状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateOpenapiStatusReq>,
    ) -> impl Responder {
        let openapi_service: OpenapiService = provider.provide();
        let resp = openapi_service.status(*id, data.status.clone() as i8).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除OpenApi接口
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let openapi_service: OpenapiService = provider.provide();
        let resp = openapi_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
