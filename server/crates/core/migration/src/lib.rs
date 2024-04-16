pub use sea_orm_migration::prelude::*;

mod m20230617_084425_create_log_api_operation;
mod m20230617_084425_create_log_system;
mod m20230617_084425_create_log_user_login;
mod m20240216_123914_create_perm_user;
mod m20240216_123914_create_perm_user_role_rel;
mod m20240218_145452_create_perm_role;
mod m20240218_145453_create_perm_dept;
mod m20240218_145453_create_perm_dept_role_rel;
mod m20240218_145453_create_perm_menu;
mod m20240218_145453_create_perm_menu_role_rel;
mod m20240218_145453_create_perm_user_token;
mod m20240218_161916_create_sys_captcha;
mod m20240218_161916_create_sys_config;
mod m20240218_161916_create_sys_dict_data;
mod m20240218_161916_create_sys_dict_dim;
mod m20240218_161916_create_sys_icon;
mod m20240415_161916_create_schedule_job;
mod m20240415_161916_create_schedule_job_log;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230617_084425_create_log_api_operation::Migration),
            Box::new(m20240415_161916_create_schedule_job_log::Migration),
            Box::new(m20230617_084425_create_log_system::Migration),
            Box::new(m20230617_084425_create_log_user_login::Migration),
            Box::new(m20240218_145452_create_perm_role::Migration),
            Box::new(m20240216_123914_create_perm_user::Migration),
            Box::new(m20240216_123914_create_perm_user_role_rel::Migration),
            Box::new(m20240218_145453_create_perm_dept::Migration),
            Box::new(m20240218_145453_create_perm_dept_role_rel::Migration),
            Box::new(m20240218_145453_create_perm_menu::Migration),
            Box::new(m20240218_145453_create_perm_menu_role_rel::Migration),
            Box::new(m20240218_145453_create_perm_user_token::Migration),
            Box::new(m20240218_161916_create_sys_captcha::Migration),
            Box::new(m20240218_161916_create_sys_config::Migration),
            Box::new(m20240218_161916_create_sys_dict_dim::Migration),
            Box::new(m20240218_161916_create_sys_dict_data::Migration),
            Box::new(m20240415_161916_create_schedule_job::Migration),
        ]
    }
}
