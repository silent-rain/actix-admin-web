pub use sea_orm_migration::prelude::*;

mod m20230617_084425_create_log_system;
mod m20230617_084425_create_log_user_login;
mod m20240216_123914_create_perm_user;
mod m20240218_145453_create_perm_dept;
mod m20240218_145452_create_perm_role;
mod m20240218_145453_create_perm_dept_role_rel;
mod m20240216_123914_create_perm_user_role_rel;
mod m20240218_161916_create_sys_captcha;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230617_084425_create_log_system::Migration),
            Box::new(m20230617_084425_create_log_user_login::Migration),
            Box::new(m20240216_123914_create_perm_user::Migration),
            Box::new(m20240218_145452_create_perm_role::Migration),
            Box::new(m20240218_145453_create_perm_dept::Migration),
            Box::new(m20240216_123914_create_perm_user_role_rel::Migration),
            Box::new(m20240218_145453_create_perm_dept_role_rel::Migration),
            Box::new(m20240218_161916_create_sys_captcha::Migration),
        ]
    }
}
