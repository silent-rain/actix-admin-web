//! 用户地理位置管理

use crate::{
    dto::location::{AddLocationReq, GetLocationListReq, UpdateLocationReq},
    service::location::LocationService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct LocationController;

impl LocationController {
    /// 获取用户地理位置列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetLocationListReq>,
    ) -> impl Responder {
        let location_service: LocationService = provider.provide();
        let resp = location_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取用户地理位置信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let location_service: LocationService = provider.provide();
        let resp = location_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加用户地理位置
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddLocationReq>,
    ) -> impl Responder {
        let location_service: LocationService = provider.provide();
        let resp = location_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户地理位置
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateLocationReq>,
    ) -> impl Responder {
        let location_service: LocationService = provider.provide();
        let resp = location_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除用户地理位置
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let location_service: LocationService = provider.provide();
        let resp = location_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
