use sea_orm_migration::{async_trait, MigrationTrait, MigratorTrait};

mod m20230617_084425_create_log_api_operation;
mod m20230617_084425_create_log_system;
mod m20230617_084425_create_log_user_login;
mod m20240218_145453_create_perm_department;
mod m20240218_145453_create_perm_department_role_rel;
mod m20240218_145453_create_perm_menu;
mod m20240218_145453_create_perm_menu_role_rel;
mod m20240218_145453_create_perm_openapi;
mod m20240218_145453_create_perm_openapi_role_rel;
mod m20240218_145453_create_perm_role;
mod m20240218_145453_create_perm_token;
mod m20240218_145453_create_perm_token_role_rel;
mod m20240218_145453_create_perm_user_role_rel;
mod m20240218_145453_create_user_base;
mod m20240218_145453_create_user_email;
mod m20240218_145453_create_user_phone;
mod m20240218_161916_create_sys_captcha;
mod m20240218_161916_create_sys_config;
mod m20240218_161916_create_sys_dict_data;
mod m20240218_161916_create_sys_dict_dim;
mod m20240218_161916_create_sys_image;
mod m20240415_161916_create_schedule_job;
mod m20240415_161916_create_schedule_job_event_log;
mod m20240415_161916_create_schedule_job_status_log;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230617_084425_create_log_api_operation::Migration),
            Box::new(m20230617_084425_create_log_system::Migration),
            Box::new(m20230617_084425_create_log_user_login::Migration),
            Box::new(m20240218_145453_create_perm_role::Migration),
            Box::new(m20240218_145453_create_user_base::Migration),
            Box::new(m20240218_145453_create_user_email::Migration),
            Box::new(m20240218_145453_create_user_phone::Migration),
            Box::new(m20240218_145453_create_perm_user_role_rel::Migration),
            Box::new(m20240218_145453_create_perm_department::Migration),
            Box::new(m20240218_145453_create_perm_department_role_rel::Migration),
            Box::new(m20240218_145453_create_perm_menu::Migration),
            Box::new(m20240218_145453_create_perm_menu_role_rel::Migration),
            Box::new(m20240218_145453_create_perm_openapi::Migration),
            Box::new(m20240218_145453_create_perm_openapi_role_rel::Migration),
            Box::new(m20240218_145453_create_perm_token::Migration),
            Box::new(m20240218_145453_create_perm_token_role_rel::Migration),
            Box::new(m20240218_161916_create_sys_captcha::Migration),
            Box::new(m20240218_161916_create_sys_config::Migration),
            Box::new(m20240218_161916_create_sys_dict_dim::Migration),
            Box::new(m20240218_161916_create_sys_dict_data::Migration),
            Box::new(m20240218_161916_create_sys_image::Migration),
            Box::new(m20240415_161916_create_schedule_job::Migration),
            Box::new(m20240415_161916_create_schedule_job_status_log::Migration),
            Box::new(m20240415_161916_create_schedule_job_event_log::Migration),
        ]
    }
}
