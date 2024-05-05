//! 图片

use crate::{
    constant::HEADERS_X_IMG,
    {
        dto::image::{
            BatchDeleteIconReq, GetIconListReq, UpdateIconReq, UploadFileForm, UploadFilesForm,
        },
        service::image::ImageService,
    },
};

use actix_multipart::form::MultipartForm;
use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};

/// 控制器
pub struct ImageController;

impl ImageController {
    /// 获取图片列表
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

    /// 获取图片信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.info(*id).await;
        match resp {
            Ok(v) => HttpResponse::Ok()
                .insert_header((HEADERS_X_IMG, "true"))
                .content_type(v.img_type)
                .body(v.base_img.to_vec()),
            Err(_err) => HttpResponse::NotFound().finish(),
        }
    }

    /// 通过hash值获取图片
    pub async fn info_by_hash(
        provider: Data<AInjectProvider>,
        hash: Path<String>,
    ) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.info_by_hash(hash.to_string()).await;
        match resp {
            Ok(v) => HttpResponse::Ok()
                .insert_header((HEADERS_X_IMG, "true"))
                .content_type(v.img_type)
                .body(v.base_img.to_vec()),
            Err(_err) => HttpResponse::NotFound().finish(),
        }
    }

    /// 上传图片
    pub async fn upload_file(
        provider: Data<AInjectProvider>,
        MultipartForm(form): MultipartForm<UploadFileForm>,
    ) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.upload_file(form).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量上传图片
    pub async fn upload_files(
        provider: Data<AInjectProvider>,
        MultipartForm(form): MultipartForm<UploadFilesForm>,
    ) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.upload_files(form).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 更新图片
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

    /// 删除图片
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let icon_service: ImageService = provider.provide();
        let resp = icon_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除图片
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