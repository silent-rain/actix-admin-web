//! 职级表
//! Entity: [`entity::organization::Rank`]

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
                    .table(Rank::Table)
                    .comment("职级表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rank::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("职级ID"),
                    )
                    .col(
                        ColumnDef::new(Rank::Name)
                            .string()
                            .string_len(20)
                            .unique_key()
                            .not_null()
                            .comment("职级名称"),
                    )
                    .col(
                        ColumnDef::new(Rank::Level)
                            .integer()
                            .unsigned()
                            .unique_key()
                            .not_null()
                            .comment("职级等级"),
                    )
                    .col(
                        ColumnDef::new(Rank::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Rank::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("职级描述"),
                    )
                    .col(
                        ColumnDef::new(Rank::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态(0:停用,1:正常)"),
                    )
                    .col(
                        ColumnDef::new(Rank::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Rank::UpdatedAt)
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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Rank::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Rank {
    #[sea_orm(iden = "t_org_rank")]
    Table,
    Id,
    Name,
    Level,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
