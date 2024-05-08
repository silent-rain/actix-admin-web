//! 路由层

pub mod user_base;
pub mod user_email;
pub mod user_phone;
pub mod user_role_rel;

use actix_web::{web, Scope};

/// 路由器
pub struct UserRouter;

impl UserRouter {
    /// 注册`用户信息管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/user")
            // 用户信息管理
            .service(user_base::UserBaseRouter::admin_register())
            // 用户手机号管理
            .service(user_phone::UserPhoneRouter::admin_register())
            // 用户邮箱管理
            .service(user_email::UserEmailRouter::admin_register())
            // 用户角色关系管理
            .service(user_role_rel::UserRoleRelRouter::admin_register())
    }
}
