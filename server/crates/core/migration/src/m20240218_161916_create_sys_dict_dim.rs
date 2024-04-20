//! 字典维度表

use sea_orm::{
    sea_query::{ColumnDef, Expr, Index, Table},
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
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
                    .table(SysDictDim::Table)
                    .comment("字典维度表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysDictDim::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("字典维度ID"),
                    )
                    .col(
                        ColumnDef::new(SysDictDim::Name)
                            .string()
                            .string_len(64)
                            .unique_key()
                            .not_null()
                            .comment("字典维度名称"),
                    )
                    .col(
                        ColumnDef::new(SysDictDim::Code)
                            .string()
                            .string_len(64)
                            .unique_key()
                            .not_null()
                            .comment("字典维度编码"),
                    )
                    .col(
                        ColumnDef::new(SysDictDim::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(SysDictDim::Note)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(SysDictDim::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(SysDictDim::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(SysDictDim::UpdatedAt)
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
            .await?;

        if !manager
            .has_index(SysDictDim::Table.to_string(), "uk_name")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("uk_name")
                        .table(SysDictDim::Table)
                        .col(SysDictDim::Name)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(SysDictDim::Table.to_string(), "uk_code")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("uk_code")
                        .table(SysDictDim::Table)
                        .col(SysDictDim::Code)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SysDictDim::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysDictDim {
    #[sea_orm(iden = "t_sys_dict_dim")]
    Table,
    Id,
    Name,
    Code,
    Sort,
    Note,
    Status,
    CreatedAt,
    UpdatedAt,
}
