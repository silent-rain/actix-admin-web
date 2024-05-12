use sea_orm_migration::{async_trait, MigrationTrait, MigratorTrait};

mod m20230617_084425_create_log_api_operation;
mod m20230617_084425_create_log_log_web;
mod m20230617_084425_create_log_system;
mod m20230617_084425_create_log_user_login;
mod m20240218_145453_create_org_department;
mod m20240218_145453_create_org_department_role_rel;
mod m20240218_145453_create_org_position;
mod m20240218_145453_create_org_rank;
mod m20240218_145453_create_perm_menu;
mod m20240218_145453_create_perm_menu_role_rel;
mod m20240218_145453_create_perm_openapi;
mod m20240218_145453_create_perm_openapi_role_rel;
mod m20240218_145453_create_perm_token;
mod m20240218_145453_create_perm_token_role_rel;
mod m20240218_145453_create_user_base;
mod m20240218_145453_create_user_blockchain_wallet;
mod m20240218_145453_create_user_email;
mod m20240218_145453_create_user_location;
mod m20240218_145453_create_user_member_level;
mod m20240218_145453_create_user_phone;
mod m20240218_145453_create_user_role;
mod m20240218_145453_create_user_role_rel;
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
            // 用户管理
            Box::new(m20240218_145453_create_user_role::Migration),
            Box::new(m20240218_145453_create_user_base::Migration),
            Box::new(m20240218_145453_create_user_email::Migration),
            Box::new(m20240218_145453_create_user_phone::Migration),
            Box::new(m20240218_145453_create_user_role_rel::Migration),
            Box::new(m20240218_145453_create_user_blockchain_wallet::Migration),
            Box::new(m20240218_145453_create_user_member_level::Migration),
            Box::new(m20240218_145453_create_user_location::Migration),
            // 权限管理
            Box::new(m20240218_145453_create_perm_menu::Migration),
            Box::new(m20240218_145453_create_perm_menu_role_rel::Migration),
            Box::new(m20240218_145453_create_perm_openapi::Migration),
            Box::new(m20240218_145453_create_perm_openapi_role_rel::Migration),
            Box::new(m20240218_145453_create_perm_token::Migration),
            Box::new(m20240218_145453_create_perm_token_role_rel::Migration),
            // 组织管理
            Box::new(m20240218_145453_create_org_department::Migration),
            Box::new(m20240218_145453_create_org_department_role_rel::Migration),
            Box::new(m20240218_145453_create_org_position::Migration),
            Box::new(m20240218_145453_create_org_rank::Migration),
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
            Box::new(m20230617_084425_create_log_user_login::Migration),
            Box::new(m20230617_084425_create_log_log_web::Migration),
        ]
    }
}
