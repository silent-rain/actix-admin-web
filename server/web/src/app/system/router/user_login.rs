//! 登陆日志

use crate::app::system::UserLoginController;

use actix_web::{web, Scope};

/// 路由
pub struct UserLoginRouter;

impl UserLoginRouter {
    /// 注册登陆日志管理路由
    pub fn register() -> Scope {
        web::scope("/system")
            // 登陆日志管理
            .service(
                web::scope("/user_login")
                    .route("/list", web::get().to(UserLoginController::list))
                    .route("", web::get().to(UserLoginController::info))
                    // .route("", web::post().to(UserLoginController::add))
                    .route(
                        "/disbale",
                        web::put().to(UserLoginController::disbale_status),
                    ),
            )
    }
}
