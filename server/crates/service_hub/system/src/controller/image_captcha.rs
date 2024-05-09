//! 图片验证码管理

use crate::{
    dto::image_captcha::{BatchDeleteImageCaptchaReq, GetImageCaptchaListReq},
    service::image_captcha::ImageCaptchaService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct ImageCaptchaController;

impl ImageCaptchaController {
    /// 获取验证码列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetImageCaptchaListReq>,
    ) -> impl Responder {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let resp = image_captcha_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取验证码信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let resp = image_captcha_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取验证码信息
    pub async fn info_by_captcha_id(
        provider: Data<AInjectProvider>,
        captcha_id: Path<String>,
    ) -> impl Responder {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let resp = image_captcha_service
            .info_by_captcha_id(captcha_id.to_string())
            .await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加验证码
    pub async fn add(provider: Data<AInjectProvider>) -> impl Responder {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let resp = image_captcha_service.add().await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 删除验证码
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let resp = image_captcha_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除验证码
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteImageCaptchaReq>,
    ) -> impl Responder {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let resp = image_captcha_service.batch_delete(data.ids.clone()).await;
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
