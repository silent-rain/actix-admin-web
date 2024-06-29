//! 岗位管理

use crate::{
    dto::position::{
        AddPositionReq, GetPositionListReq, UpdatePositionReq, UpdatePositionStatusReq,
    },
    service::position::PositionService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct PositionController;

impl PositionController {
    /// 获取岗位列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetPositionListReq>,
    ) -> impl Responder {
        let position_service: PositionService = provider.provide();
        let resp = position_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取岗位信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let position_service: PositionService = provider.provide();
        let resp = position_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加岗位
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddPositionReq>,
    ) -> impl Responder {
        let position_service: PositionService = provider.provide();
        let resp = position_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新岗位
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdatePositionReq>,
    ) -> impl Responder {
        let position_service: PositionService = provider.provide();
        let resp = position_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新岗位状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdatePositionStatusReq>,
    ) -> impl Responder {
        let position_service: PositionService = provider.provide();
        let resp = position_service
            .status(*id, data.status.clone() as i8)
            .await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除岗位
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let position_service: PositionService = provider.provide();
        let resp = position_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
