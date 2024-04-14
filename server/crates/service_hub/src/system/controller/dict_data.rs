//! 字典数据管理

use crate::{
    inject::AInjectProvider,
    system::{
        dto::dict_data::{
            AddDictDataReq, GetDictDataListReq, UpdateDictDataReq, UpdateDictDataStatusReq,
        },
        service::dict_data::DictDataService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct DictDataController;

impl DictDataController {
    /// 获取字典数据列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetDictDataListReq>,
    ) -> impl Responder {
        let dict_data_service: DictDataService = provider.provide();
        let resp = dict_data_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取字典数据信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let dict_data_service: DictDataService = provider.provide();
        let resp = dict_data_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加字典数据
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddDictDataReq>,
    ) -> impl Responder {
        let dict_data_service: DictDataService = provider.provide();
        let resp = dict_data_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新字典数据
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateDictDataReq>,
    ) -> impl Responder {
        let dict_data_service: DictDataService = provider.provide();
        let resp = dict_data_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新字典数据状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateDictDataStatusReq>,
    ) -> impl Responder {
        let dict_data_service: DictDataService = provider.provide();
        let resp = dict_data_service.status(*id, data.status).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除字典数据
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let dict_data_service: DictDataService = provider.provide();
        let resp = dict_data_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
