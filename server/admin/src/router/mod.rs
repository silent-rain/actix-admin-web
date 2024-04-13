//! 路由集散处, 将各个模块的路由在此处进行注册。
use context::ContextMiddleware;
use service_hub::{
    auth::{GenCaptchaRouter, LoginRouter, RegisterRouter},
    log::LogRouter,
    perm::{DeptRoleRelRouter, DeptRouter, RoleRouter, UserRoleRelRouter, UserRouter},
    public::HealthRouter,
    system::SystemRouter,
};

use actix_web::{dev::HttpServiceFactory, web};
use tracing_actix_web::TracingLogger;

/// 注册路由
///
/// Service Hub Module: [`service_hub`]
pub fn register() -> impl HttpServiceFactory {
    web::scope("/api/v1")
        // >>> 中间件 >>>
        // 注意中间件加载顺序: Last in, first loading
        .wrap(TracingLogger::default())
        .wrap(middleware::cors::wrap_cors())
        // 接口鉴权
        .wrap(middleware::auth::Auth)
        // 上下文中间件
        .wrap(ContextMiddleware)
        // <<< 中间件 <<<
        // 健康检查
        .service(HealthRouter::register())
        // 生成验证码
        .service(GenCaptchaRouter::register())
        // 登陆
        .service(LoginRouter::register())
        // 注册用户
        .service(RegisterRouter::register())
        // 后台管理接口
        .service(
            web::scope("/admin")
                // 角色管理
                .service(RoleRouter::admin_register())
                // 用户管理
                .service(UserRouter::admin_register())
                // 用户角色关系管理
                .service(UserRoleRelRouter::admin_register())
                // 部门管理
                .service(DeptRouter::admin_register())
                // 部门角色关系管理
                .service(DeptRoleRelRouter::admin_register())
                // 系统管理
                .service(SystemRouter::admin_register())
                // 日志管理
                .service(LogRouter::admin_register()),
        )
}
