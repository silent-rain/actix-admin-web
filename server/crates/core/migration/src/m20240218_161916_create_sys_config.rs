//! 配置表
//! User Entity: [`entity::prelude::SysConfig`]

use sea_orm::{
    sea_query::{ColumnDef, Expr, Table},
    DatabaseBackend, DeriveIden, DeriveMigrationName,
};
use sea_orm_migration::{async_trait, DbErr, MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(SysConfig::Table)
                    .comment("配置表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysConfig::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("配置ID"),
                    )
                    .col(
                        ColumnDef::new(SysConfig::Pid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("父节点ID"),
                    )
                    .col(
                        ColumnDef::new(SysConfig::Name)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("配置名称"),
                    )
                    .col(
                        ColumnDef::new(SysConfig::Code)
                            .string()
                            .string_len(64)
                            .unique_key()
                            .not_null()
                            .comment("配置编码(英文)"),
                    )
                    .col(
                        ColumnDef::new(SysConfig::Value)
                            .text()
                            .null()
                            .comment("配置值"),
                    )
                    .col(
                        ColumnDef::new(SysConfig::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(SysConfig::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("配置描述"),
                    )
                    .col(
                        ColumnDef::new(SysConfig::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(SysConfig::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(SysConfig::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra({
                                match manager.get_database_backend() {
                                    DatabaseBackend::Sqlite => "DEFAULT CURRENT_TIMESTAMP",
                                    _ => "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP",
                                }
                            })
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SysConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysConfig {
    #[sea_orm(iden = "t_sys_config")]
    Table,
    Id,
    Pid,
    Name,
    Code,
    Value,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
