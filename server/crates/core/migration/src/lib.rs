use sea_orm_migration::{async_trait, MigrationTrait, MigratorTrait};

mod log;
mod organization;
mod permission;
mod schedule;
mod system;
mod user;

pub mod template;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // 应用模板表
            Box::new(template::app_template::Migration),
            // 用户管理
            Box::new(user::user_base::Migration),
            Box::new(user::phone::Migration),
            Box::new(user::email::Migration),
            Box::new(user::blockchain_wallet::Migration),
            Box::new(user::role::Migration),
            Box::new(user::user_role_rel::Migration),
            Box::new(user::member_level::Migration),
            Box::new(user::location::Migration),
            Box::new(user::user_login_log::Migration),
            // 权限管理
            Box::new(permission::menu::Migration),
            Box::new(permission::menu_role_rel::Migration),
            Box::new(permission::openapi::Migration),
            Box::new(permission::openapi_role_rel::Migration),
            Box::new(permission::token::Migration),
            Box::new(permission::token_role_rel::Migration),
            // 组织管理
            Box::new(organization::department::Migration),
            Box::new(organization::department_role_rel::Migration),
            Box::new(organization::position::Migration),
            Box::new(organization::rank::Migration),
            // 系统管理
            Box::new(system::sys_config::Migration),
            Box::new(system::sys_dict_dimension::Migration),
            Box::new(system::sys_dict_data::Migration),
            Box::new(system::sys_image_captcha::Migration),
            Box::new(system::sys_image_resource::Migration),
            // 任务调度作业管理
            Box::new(schedule::schedule_job::Migration),
            Box::new(schedule::schedule_status_log::Migration),
            Box::new(schedule::schedule_event_log::Migration),
            // 日志管理
            Box::new(log::log_api_operation::Migration),
            Box::new(log::log_system::Migration),
            Box::new(log::log_web::Migration),
        ]
    }
}
