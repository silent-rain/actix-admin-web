//! 应用服务
use crate::{
    app::{
        auth::{LoginRouter, RegisterRouter},
        log::SystemRouter,
        perm::{RoleRouter, UserRoleRelRouter, UserRouter},
        public::HealthRouter,
        system::UserLoginRouter,
    },
    middleware,
};

use context::ContextMiddleware;

use actix_request_identifier::RequestIdentifier;
use actix_web::middleware::Logger;
use actix_web::{dev::HttpServiceFactory, web};
use actix_web_requestid::RequestIDMiddleware;
use tracing_actix_web::TracingLogger;

/// API 服务
/// 注册路由
pub fn register() -> impl HttpServiceFactory {
    web::scope("/api/v1")
        // >>> 中间件 >>>
        .wrap(Logger::default())
        .wrap(TracingLogger::default())
        .wrap(ContextMiddleware)
        .wrap(middleware::cors::wrap_cors())
        // Request ID
        .wrap(RequestIDMiddleware::default())
        // Actix Request Identifier
        .wrap(RequestIdentifier::with_uuid())
        // .wrap(middleware::auth::SayHi)
        // <<< 中间件 <<<
        // 打招呼
        .service(HealthRouter::register())
        // 登陆
        .service(LoginRouter::register())
        // 注册用户
        .service(RegisterRouter::register())
        // 用户管理
        .service(UserRouter::register())
        // 角色管理
        .service(RoleRouter::register())
        // 用户角色关联关系管理
        .service(UserRoleRelRouter::register())
        // 系统日志管理
        .service(SystemRouter::register())
        // 登陆日志管理
        .service(UserLoginRouter::register())
}
