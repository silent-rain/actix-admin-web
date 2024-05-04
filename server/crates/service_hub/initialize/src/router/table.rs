//! 库表初始化

use crate::TableController;

use actix_web::{web, Scope};

/// 路由器
pub struct TableRouter;

impl TableRouter {
    /// 注册`库表初始化`路由
    pub fn admin_register() -> Scope {
        web::scope("/table").route("", web::post().to(TableController::table))
    }
}
