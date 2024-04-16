//! 用户Token令牌表
//! User Entity: [`entity::prelude::PermUserToken`]
use entity::{perm_user_token::Column, prelude::PermUserToken};

use sea_orm_migration::{
    async_trait,
    sea_orm::DeriveMigrationName,
    sea_query::{ColumnDef, Expr, Table},
    DbErr, MigrationTrait, SchemaManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(PermUserToken)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("令牌ID"),
                    )
                    .col(
                        ColumnDef::new(Column::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Token)
                            .string()
                            .string_len(50)
                            .unique_key()
                            .not_null()
                            .comment("令牌"),
                    )
                    .col(
                        ColumnDef::new(Column::Passphrase)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("口令"),
                    )
                    .col(
                        ColumnDef::new(Column::Permission)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("权限范围:GET,POST,PUT,DELETE"),
                    )
                    .col(
                        ColumnDef::new(Column::Expire)
                            .date_time()
                            .not_null()
                            .comment("授权到期时间"),
                    )
                    .col(
                        ColumnDef::new(Column::Note)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(Column::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(PermUserToken).to_owned())
            .await
    }
}
