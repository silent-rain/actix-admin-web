//! 字典维度表
//! Entity: [`entity::prelude::SysDictDimension`]
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
                    .table(SysDictDimension::Table)
                    .comment("字典维度表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysDictDimension::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("字典维度ID"),
                    )
                    .col(
                        ColumnDef::new(SysDictDimension::Name)
                            .string()
                            .string_len(64)
                            .unique_key()
                            .not_null()
                            .comment("字典维度名称"),
                    )
                    .col(
                        ColumnDef::new(SysDictDimension::Code)
                            .string()
                            .string_len(64)
                            .unique_key()
                            .not_null()
                            .comment("字典维度编码"),
                    )
                    .col(
                        ColumnDef::new(SysDictDimension::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(SysDictDimension::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(SysDictDimension::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态(0:停用,1:正常)"),
                    )
                    .col(
                        ColumnDef::new(SysDictDimension::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(SysDictDimension::UpdatedAt)
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
            .has_index(SysDictDimension::Table.to_string(), "uk_name")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(SysDictDimension::Table)
                        .name("uk_name")
                        .unique()
                        .col(SysDictDimension::Name)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(SysDictDimension::Table.to_string(), "uk_code")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("uk_code")
                        .unique()
                        .table(SysDictDimension::Table)
                        .col(SysDictDimension::Code)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SysDictDimension::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysDictDimension {
    #[sea_orm(iden = "t_sys_dict_dimension")]
    Table,
    Id,
    Name,
    Code,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
