//! 字典维度管理

use crate::{
    dto::dict_dimension::{
        AddDictDimensionReq, GetDictDimensionListReq, UpdateDictDimensionReq,
        UpdateDictDimensionStatusReq,
    },
    service::dict_dimension::DictDimensionService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct DictDimensionController;

impl DictDimensionController {
    /// 获取字典维度列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetDictDimensionListReq>,
    ) -> impl Responder {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let resp = dict_dimension_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取字典维度信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let resp = dict_dimension_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加字典维度
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddDictDimensionReq>,
    ) -> impl Responder {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let resp = dict_dimension_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新字典维度
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateDictDimensionReq>,
    ) -> impl Responder {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let resp = dict_dimension_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新字典维度状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateDictDimensionStatusReq>,
    ) -> impl Responder {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let resp = dict_dimension_service
            .status(*id, data.status.clone() as i8)
            .await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除字典维度
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let resp = dict_dimension_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
