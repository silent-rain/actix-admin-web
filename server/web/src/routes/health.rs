//! 健康检查

use crate::controller::health::Controller;

use actix_web::{web, Scope};

/// 注册路由
pub fn register() -> Scope {
    web::scope("/health").route("", web::get().to(Controller::health))
}
