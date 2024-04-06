//! Context extractor.

use crate::Context;

use actix_web::{dev::Payload, error::ErrorNotExtended, FromRequest, HttpMessage, HttpRequest};
use futures::future::{err, ok, Ready};

impl FromRequest for Context {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let binding = req.extensions();
        let ctx = match binding.get::<Context>() {
            Some(v) => v,
            None => return err(ErrorNotExtended("no found Context")),
        };

        ok(ctx.clone())
    }
}
