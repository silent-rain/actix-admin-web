//! 应用服务
use crate::{
    app::{
        auth::{LoginRouter, RegisterRouter},
        log::LogRouter,
        perm::{RoleRouter, UserRoleRelRouter, UserRouter},
        public::HealthRouter,
        system::CaptchaRouter,
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
        // 健康检查
        .service(HealthRouter::register())
        // 登陆
        .service(LoginRouter::register())
        // 注册用户
        .service(RegisterRouter::register())
        // 后台管理接口
        .service(
            web::scope("/admin")
                // 用户管理
                .service(UserRouter::admin_register())
                // 角色管理
                .service(RoleRouter::admin_register())
                // 用户角色关联关系管理
                .service(UserRoleRelRouter::admin_register())
                // 验证码管理
                .service(CaptchaRouter::admin_register())
                // 日志管理
                .service(LogRouter::admin_register()),
        )
}
