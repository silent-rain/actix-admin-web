//! 配置管理

use crate::{
    inject::AInjectProvider,
    system::{
        dto::config::{AddConfigReq, GetConfigListReq, UpdateConfigReq, UpdateConfigStatusReq},
        service::config::ConfigService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct ConfigController;

impl ConfigController {
    /// 获配置色列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetConfigListReq>,
    ) -> impl Responder {
        let config_service: ConfigService = provider.provide();
        let resp = config_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获配置色信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let config_service: ConfigService = provider.provide();
        let resp = config_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加配置
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddConfigReq>) -> impl Responder {
        let config_service: ConfigService = provider.provide();
        let resp = config_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新配置
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateConfigReq>,
    ) -> impl Responder {
        let config_service: ConfigService = provider.provide();
        let resp = config_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新配置状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateConfigStatusReq>,
    ) -> impl Responder {
        let config_service: ConfigService = provider.provide();
        let resp = config_service.status(*id, data.status).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除配置
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let config_service: ConfigService = provider.provide();
        let resp = config_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
