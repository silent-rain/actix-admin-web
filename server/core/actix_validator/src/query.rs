//! Json extractor.
use core::fmt::Debug;
use std::ops::Deref;

use response::Response;

use actix_web::{dev::Payload, web, FromRequest, HttpRequest, HttpResponse};
use futures::future::{err, ok, Ready};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug)]
pub struct Query<T>(pub T);

#[allow(unused)]
impl<T> Query<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Query<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> FromRequest for Query<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // 参数校验
        web::QueryConfig::default().error_handler(|e, _req| {
            // 自定义错误处理
            actix_web::error::InternalError::from_response(e, HttpResponse::BadRequest().finish())
                .into()
        });

        // 解析查询字符串
        let query_string = req.query_string();
        // 从查询字符串中解析出 T 结构体
        let query_info: Result<T, _> = serde_urlencoded::from_str(query_string);
        // 根据解析结果进行验证
        let inner_query = match query_info {
            Ok(v) => v,
            Err(e) => {
                let resp =
                    Response::build().code(code::Error::RequestParameterParseError(e.to_string()));
                return err(resp.into());
            }
        };

        // 验证字段
        if let Err(e) = inner_query.validate() {
            let resp = Response::build().code(code::Error::RequestParameterParseError(
                e.to_owned().to_string(),
            ));
            return err(resp.into());
        }
        ok(Query(inner_query))
    }
}
