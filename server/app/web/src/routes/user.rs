//!用户管理
use crate::controller::user;

use actix_web::{web, Scope};

pub fn register_routes() -> Scope {
    web::scope("/user")
        .route("list", web::get().to(user::Controller::list))
        .route("info", web::get().to(user::Controller::info))
        .route("add", web::get().to(user::Controller::add))
}
