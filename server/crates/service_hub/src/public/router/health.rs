//! 健康检查

use crate::public::HealthController;

use actix_web::{web, Scope};

/// 路由
pub struct HealthRouter;

impl HealthRouter {
    /// 注册路由
    pub fn register() -> Scope {
        web::scope("/health").route("", web::get().to(HealthController::health))
    }
}
