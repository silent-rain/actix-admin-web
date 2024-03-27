//! 验证码

use crate::{
    app::system::{
        dto::captcha::{AddCaptchaReq, CaptchaInfoReq, CaptchaListReq, DeleteCaptchaReq},
        service::captcha::CaptchaService,
    },
    inject::AProvider,
};

use actix_validator::{Json, Query};
use code::Error;
use response::Response;
use utils::captcha::generate_captcha;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct CaptchaController;

impl CaptchaController {
    /// 列表
    pub async fn list(provider: Data<AProvider>, req: Query<CaptchaListReq>) -> impl Responder {
        let captcha_service: CaptchaService = provider.provide();
        let resp = captcha_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data_list(results, total)
    }

    /// 获取验证
    pub async fn info(provider: Data<AProvider>, params: Query<CaptchaInfoReq>) -> impl Responder {
        // 生成验证码
        generate_captcha();

        let captcha_service: CaptchaService = provider.provide();
        let resp = captcha_service.info(params.id).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };
        let result = match result {
            Some(v) => v,
            None => return Response::code(Error::DbQueryEmptyError),
        };

        Response::ok().data(result)
    }

    /// 添加信息
    pub async fn add(provider: Data<AProvider>, data: Json<AddCaptchaReq>) -> impl Responder {
        let captcha_service: CaptchaService = provider.provide();
        let resp = captcha_service.add(data.into_inner()).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data(result)
    }

    /// 删除信息
    pub async fn delete(
        provider: Data<AProvider>,
        params: Query<DeleteCaptchaReq>,
    ) -> impl Responder {
        let captcha_service: CaptchaService = provider.provide();
        let resp = captcha_service.delete(params.id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().msg("删除成功")
    }
}
