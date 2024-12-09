use sea_orm_migration::{async_trait, MigrationTrait, MigratorTrait};

mod organization;
mod permission;
mod user;

pub mod m20230210_145453_create_app_template;
mod m20230617_084425_create_log_api_operation;
mod m20230617_084425_create_log_log_web;
mod m20230617_084425_create_log_system;
mod m20240218_161916_create_sys_config;
mod m20240218_161916_create_sys_dict_data;
mod m20240218_161916_create_sys_dict_dimension;
mod m20240218_161916_create_sys_image_captcha;
mod m20240218_161916_create_sys_image_resource;
mod m20240415_161916_create_schedule_event_log;
mod m20240415_161916_create_schedule_job;
mod m20240415_161916_create_schedule_status_log;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // 应用模板表
            // Box::new(m20230210_145453_create_app_template::Migration),
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
            Box::new(m20240218_161916_create_sys_config::Migration),
            Box::new(m20240218_161916_create_sys_dict_dimension::Migration),
            Box::new(m20240218_161916_create_sys_dict_data::Migration),
            Box::new(m20240218_161916_create_sys_image_captcha::Migration),
            Box::new(m20240218_161916_create_sys_image_resource::Migration),
            // 任务调度作业管理
            Box::new(m20240415_161916_create_schedule_job::Migration),
            Box::new(m20240415_161916_create_schedule_status_log::Migration),
            Box::new(m20240415_161916_create_schedule_event_log::Migration),
            // 日志管理
            Box::new(m20230617_084425_create_log_api_operation::Migration),
            Box::new(m20230617_084425_create_log_system::Migration),
            Box::new(m20230617_084425_create_log_log_web::Migration),
        ]
    }
}
