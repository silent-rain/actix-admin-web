//! OpenApi接口管理

use crate::{
    inject::AInjectProvider,
    perm::{
        dto::open_api::{
            AddOpenApiReq, GetOpenApiListReq, UpdateOpenApiReq, UpdateOpenApiStatusReq,
        },
        service::open_api::OpenApiService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct OpenApiController;

impl OpenApiController {
    /// 获取OpenApi接口列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetOpenApiListReq>,
    ) -> impl Responder {
        let dept_service: OpenApiService = provider.provide();
        let resp = dept_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取OpenApi接口树列表
    pub async fn tree(provider: Data<AInjectProvider>) -> impl Responder {
        let dept_service: OpenApiService = provider.provide();
        let resp = dept_service.tree().await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取OpenApi接口信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let dept_service: OpenApiService = provider.provide();
        let resp = dept_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加OpenApi接口
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddOpenApiReq>) -> impl Responder {
        let dept_service: OpenApiService = provider.provide();
        let resp = dept_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新OpenApi接口
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateOpenApiReq>,
    ) -> impl Responder {
        let dept_service: OpenApiService = provider.provide();
        let resp = dept_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新OpenApi接口状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateOpenApiStatusReq>,
    ) -> impl Responder {
        let dept_service: OpenApiService = provider.provide();
        let resp = dept_service.status(*id, data.status).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除OpenApi接口
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let dept_service: OpenApiService = provider.provide();
        let resp = dept_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
