pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230617_084425_create_log_system_table;
mod m20240216_123914_create_perm_user_table;
mod m20240218_145452_create_perm_role_table;
mod m20240218_161914_create_perm_user_role_rel;
mod m20240218_161915_create_sys_user_login;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240216_123914_create_perm_user_table::Migration),
            Box::new(m20230617_084425_create_log_system_table::Migration),
            Box::new(m20240218_145452_create_perm_role_table::Migration),
            Box::new(m20240218_161914_create_perm_user_role_rel::Migration),
            Box::new(m20240218_161915_create_sys_user_login::Migration),
        ]
    }
}
