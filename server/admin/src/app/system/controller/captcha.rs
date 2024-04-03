//! 验证码

use crate::{
    app::system::{
        dto::captcha::{AddCaptchaResp, BatchDeleteCaptchaReq, CaptchaListReq},
        service::captcha::CaptchaService,
    },
    config::AppConfig,
    inject::AProvider,
};

use actix_validator::{Json, Query};
use actix_web::web::Path;
use entity::sys_captcha;
use response::Response;
use utils::captcha::generate_captcha;

use actix_web::{web::Data, Responder};
use uuid::Uuid;

/// 控制器
pub struct CaptchaController;

impl CaptchaController {
    /// 获取验证码列表
    pub async fn list(provider: Data<AProvider>, req: Query<CaptchaListReq>) -> impl Responder {
        let captcha_service: CaptchaService = provider.provide();
        let resp = captcha_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data_list(results, total)
    }

    /// 获取验证码信息
    pub async fn info(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let captcha_service: CaptchaService = provider.provide();
        let resp = captcha_service.info(*id).await;

        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 获取验证码信息
    pub async fn info_by_captcha_id(
        provider: Data<AProvider>,
        captcha_id: Path<String>,
    ) -> impl Responder {
        let captcha_service: CaptchaService = provider.provide();
        let resp = captcha_service
            .info_by_captcha_id(captcha_id.to_string())
            .await;

        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 添加验证码
    pub async fn add(provider: Data<AProvider>, conf: Data<AppConfig>) -> impl Responder {
        // 生成验证码
        let (captcha, base_img) = generate_captcha();
        let captcha_id = Uuid::new_v4().to_string();
        let expire = conf.server.captcha.expire;
        let data = sys_captcha::Model {
            captcha_id,
            captcha,
            base_img: base_img.into_bytes(),
            expire,
            ..Default::default()
        };

        let captcha_service: CaptchaService = provider.provide();
        let resp = captcha_service.add(data).await;

        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };
        let base_img = match String::from_utf8(result.base_img) {
            Ok(v) => v,
            Err(err) => return Response::code(code::Error::FromUtf8Error(err)),
        };
        let result = AddCaptchaResp {
            captcha_id: result.captcha_id,
            base_img,
            expire: result.expire,
            created_at: result.created_at,
        };

        Response::ok().data(result)
    }

    /// 删除验证码
    pub async fn delete(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let captcha_service: CaptchaService = provider.provide();
        let resp = captcha_service.delete(*id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("删除成功")
    }

    /// 批量删除验证码
    pub async fn batch_delete(
        provider: Data<AProvider>,
        data: Json<BatchDeleteCaptchaReq>,
    ) -> impl Responder {
        let captcha_service: CaptchaService = provider.provide();
        let resp = captcha_service.batch_delete(data.ids.clone()).await;
        let _result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("删除成功")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid() {
        let uuid = Uuid::new_v4().to_string();
        assert_eq!(uuid.len(), 36);
    }
}
