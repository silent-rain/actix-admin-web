//! 用户手机号表
//! User Entity: [`entity::prelude::PermUserPhone`]

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
                    .table(PermUserPhone::Table)
                    .comment("用户手机号表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermUserPhone::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("手机号ID"),
                    )
                    .col(
                        ColumnDef::new(PermUserPhone::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(PermUserPhone::Phone)
                            .string()
                            .string_len(16)
                            .not_null()
                            .comment("手机号码"),
                    )
                    .col(
                        ColumnDef::new(PermUserPhone::Note)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(PermUserPhone::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermUserPhone::UpdatedAt)
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
            .has_index(PermUserPhone::Table.to_string(), "uk_user_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(PermUserPhone::Table)
                        .name("uk_user_id")
                        .unique()
                        .col(PermUserPhone::UserId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(PermUserPhone::Table.to_string(), "uk_phone")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(PermUserPhone::Table)
                        .name("uk_phone")
                        .unique()
                        .col(PermUserPhone::Phone)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermUserPhone::Table.to_string(),
                "fk_perm_user_phone_user_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_user_phone_user_id")
                        .from(PermUserPhone::Table, PermUserPhone::UserId)
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
            .drop_table(Table::drop().table(PermUserPhone::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermUserPhone {
    #[sea_orm(iden = "t_perm_user_phone")]
    Table,
    Id,
    UserId,
    Phone,
    Note,
    CreatedAt,
    UpdatedAt,
}
