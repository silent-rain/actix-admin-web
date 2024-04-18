//! ICON图片

use crate::{
    inject::AInjectProvider,
    system::{
        dto::icon::{AddIconReq, BatchDeleteIconReq, GetIconListReq, UpdateIconReq},
        service::image::ImageService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct ImageController;

impl ImageController {
    /// 获取ICON图片列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetIconListReq>,
    ) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取ICON图片信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加ICON图片
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddIconReq>) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.add(data.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 更新ICON图片
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateIconReq>,
    ) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除ICON图片
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除ICON图片
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteIconReq>,
    ) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.batch_delete(data.ids.clone()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    #[test]
    fn test_uuid() {
        let uuid = Uuid::new_v4().to_string();
        assert_eq!(uuid.len(), 36);
    }
}
