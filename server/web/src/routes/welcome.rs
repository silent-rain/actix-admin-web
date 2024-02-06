//! 欢迎语

use crate::controller::welcome::Controller;

use actix_web::{web, Scope};

/// 注册路由
pub fn register() -> Scope {
    web::scope("/greet").route("", web::get().to(Controller::greet))
}
