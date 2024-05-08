//! 令牌表
//! Entity: [`entity::prelude::PermToken`]

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
                    .table(PermToken::Table)
                    .comment("令牌表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermToken::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("令牌ID"),
                    )
                    .col(
                        ColumnDef::new(PermToken::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(PermToken::Token)
                            .string()
                            .string_len(50)
                            .unique_key()
                            .not_null()
                            .comment("令牌"),
                    )
                    .col(
                        ColumnDef::new(PermToken::Passphrase)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("口令"),
                    )
                    .col(
                        ColumnDef::new(PermToken::Permission)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("权限范围:GET,POST,PUT,DELETE"),
                    )
                    .col(
                        ColumnDef::new(PermToken::Expire)
                            .date_time()
                            .not_null()
                            .comment("授权到期时间"),
                    )
                    .col(
                        ColumnDef::new(PermToken::Note)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(PermToken::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(PermToken::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermToken::UpdatedAt)
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
            .drop_table(Table::drop().table(PermToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermToken {
    #[sea_orm(iden = "t_perm_token")]
    Table,
    Id,
    UserId,
    Token,
    Passphrase,
    Permission,
    Expire,
    Status,
    Note,
    CreatedAt,
    UpdatedAt,
}
