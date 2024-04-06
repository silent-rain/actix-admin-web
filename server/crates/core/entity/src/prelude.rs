//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

pub use super::app_template::Entity as AppTemplate;

// 日志管理
pub use super::log_http::Entity as LogHttp;
pub use super::log_system::Entity as LogSystem;
pub use super::log_user_login::Entity as LogUserLogin;
pub use super::log_web::Entity as LogWeb;

pub use super::api_http::Entity as ApiHttp;
pub use super::api_role_http_rel::Entity as ApiRoleHttpRel;
pub use super::api_token::Entity as ApiToken;
pub use super::perm_menu::Entity as PermMenu;
pub use super::perm_role::Entity as PermRole;
pub use super::perm_role_menu_rel::Entity as PermRoleMenuRel;
pub use super::perm_user::Entity as PermUser;
pub use super::perm_user_api_token::Entity as PermUserApiToken;
pub use super::perm_user_role_rel::Entity as PermUserRoleRel;
pub use super::sys_captcha::Entity as SysCaptcha;
pub use super::sys_config::Entity as SysConfig;
pub use super::sys_icon::Entity as SysIcon;