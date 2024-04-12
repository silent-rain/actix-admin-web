//! Json extractor.
use core::fmt::Debug;
use std::ops::Deref;

use response::Response;

use actix_web::{dev::Payload, web, FromRequest, HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;
use serde::de::DeserializeOwned;
use tracing::error;
use validator::Validate;

#[derive(Debug)]
pub struct Json<T>(pub T);

#[allow(unused)]
impl<T> Json<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Json<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> FromRequest for Json<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // 参数校验
        web::JsonConfig::default()
            .limit(4096) // 设置 Payload 的大小限制 4kb
            .error_handler(|e, _req| {
                // 自定义错误处理
                actix_web::error::InternalError::from_response(
                    e,
                    HttpResponse::BadRequest().finish(),
                )
                .into()
            });

        let fut = web::Json::<T>::from_request(req, payload);

        Box::pin(async move {
            // 获取body数据
            let body = fut.await?;
            let inner_body = body.into_inner();

            // 验证 body 数据
            inner_body.validate().map_err(|e| {
                error!("参数验证失败, err: {e}");
                Response::err(
                    code::Error::InvalidParameterError
                        .into_msg()
                        .with_msg("参数验证失败"),
                )
            })?;
            Ok(Json(inner_body))
        })
    }
}
