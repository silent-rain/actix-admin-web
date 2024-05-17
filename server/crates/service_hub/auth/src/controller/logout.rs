//! 登出

use crate::service::logout::Logoutervice;

use context::Context;
use inject::AInjectProvider;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct LogoutController;

impl LogoutController {
    /// 登出
    pub async fn logout(ctx: Context, provider: Data<AInjectProvider>) -> impl Responder {
        let user_id = ctx.get_user_id();
        let user_login_id = ctx.get_user_login_id();
        let login_service: Logoutervice = provider.provide();
        let result = login_service.logout(user_id, user_login_id).await;
        match result {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }
}
