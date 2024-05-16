//! 应用模板表
//! Entity: [`entity::prelude::AppTemplate`]

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
                    .table(AppTemplate::Table)
                    .comment("应用模板表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppTemplate::Id)
                            .integer()
                            // .primary_key()
                            // .auto_increment()
                            .extra({
                                match manager.get_database_backend() {
                                    // `id` INTEGER  NOT NULL PRIMARY KEY AUTOINCREMENT,
                                    DatabaseBackend::Sqlite => "PRIMARY KEY AUTOINCREMENT",
                                    // `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
                                    _ => "PRIMARY KEY AUTO_INCREMENT",
                                }
                            })
                            .not_null()
                            .comment("模板ID"),
                    )
                    .col(
                        ColumnDef::new(AppTemplate::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(AppTemplate::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(AppTemplate::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态(0:停用,1:正常)"),
                    )
                    .col(
                        ColumnDef::new(AppTemplate::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(AppTemplate::UpdatedAt)
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
            .drop_table(Table::drop().table(AppTemplate::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AppTemplate {
    #[sea_orm(iden = "t_app_template")]
    Table,
    Id,
    UserId,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
