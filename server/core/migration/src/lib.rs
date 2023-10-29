pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230611_123914_create_perm_user_table;
mod m20230617_084425_create_log_system_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230611_123914_create_perm_user_table::Migration),
            Box::new(m20230617_084425_create_log_system_table::Migration),
        ]
    }
}
