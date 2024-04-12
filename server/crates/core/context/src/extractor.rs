//! Context extractor.

use crate::Context;

use actix_web::{dev::Payload, error::ErrorNotExtended, FromRequest, HttpMessage, HttpRequest};
use futures::future::{err, ok, Ready};
use tracing_actix_web::RequestId;

impl FromRequest for Context {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let binding = req.extensions();
        let mut ctx = match binding.get::<Context>().cloned() {
            Some(v) => v,
            None => return err(ErrorNotExtended("no found Context")),
        };

        // 设置接口请求UUID
        let request_id = match req.extensions().get::<RequestId>().cloned() {
            Some(v) => v.to_string(),
            None => "".to_owned(),
        };
        ctx.set_request_id(request_id);

        ok(ctx.clone())
    }
}
