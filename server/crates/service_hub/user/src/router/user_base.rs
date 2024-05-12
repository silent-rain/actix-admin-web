//! 用户信息管理

use crate::controller::user_base::UserBaseController;

use actix_web::{web, Scope};

/// 路由器
pub struct UserBaseRouter;

impl UserBaseRouter {
    /// 注册`用户信息管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/base")
            .route("/profile", web::get().to(UserBaseController::profile))
            .route("", web::get().to(UserBaseController::list))
            .route("/{id}", web::get().to(UserBaseController::info))
            .route("", web::post().to(UserBaseController::add))
            .route("/{id}", web::put().to(UserBaseController::update))
            .route(
                "/{id}/share-code",
                web::put().to(UserBaseController::update_share_code),
            )
            .route("/{id}/status", web::put().to(UserBaseController::status))
            .route("/{id}", web::delete().to(UserBaseController::delete))
            .route("/{id}/roles", web::get().to(UserBaseController::roles))
    }
}
