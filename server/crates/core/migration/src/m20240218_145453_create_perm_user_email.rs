//! 用户邮箱表
//! User Entity: [`entity::prelude::PermUserEmail`]

use crate::m20240218_145453_create_user_base::UserBase;

use sea_orm::{
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Index, Table},
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
                    .table(PermUserEmail::Table)
                    .comment("用户邮箱表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermUserEmail::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("邮箱ID"),
                    )
                    .col(
                        ColumnDef::new(PermUserEmail::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(PermUserEmail::Email)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("邮箱"),
                    )
                    .col(
                        ColumnDef::new(PermUserEmail::Note)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(PermUserEmail::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermUserEmail::UpdatedAt)
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
            .has_index(PermUserEmail::Table.to_string(), "uk_user_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(PermUserEmail::Table)
                        .name("uk_user_id")
                        .unique()
                        .col(PermUserEmail::UserId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(PermUserEmail::Table.to_string(), "uk_email")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(PermUserEmail::Table)
                        .name("uk_email")
                        .unique()
                        .col(PermUserEmail::Email)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermUserEmail::Table.to_string(),
                "fk_perm_user_email_user_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_user_email_user_id")
                        .from(PermUserEmail::Table, PermUserEmail::UserId)
                        .to(UserBase::Table, UserBase::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(PermUserEmail::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermUserEmail {
    #[sea_orm(iden = "t_perm_user_email")]
    Table,
    Id,
    UserId,
    Email,
    Note,
    CreatedAt,
    UpdatedAt,
}
