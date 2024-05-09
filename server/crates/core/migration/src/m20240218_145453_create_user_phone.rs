//! 用户手机号表
//! Entity: [`entity::prelude::UserPhone`]

use crate::m20240218_145453_create_user_base::UserBase;

use sea_orm::{
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Table},
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
                    .table(UserPhone::Table)
                    .comment("用户手机号表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserPhone::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("手机号ID"),
                    )
                    .col(
                        ColumnDef::new(UserPhone::UserId)
                            .integer()
                            .unique_key()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserPhone::Phone)
                            .string()
                            .string_len(16)
                            .unique_key()
                            .not_null()
                            .comment("手机号码"),
                    )
                    .col(
                        ColumnDef::new(UserPhone::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(UserPhone::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserPhone::UpdatedAt)
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
            .has_index(UserPhone::Table.to_string(), "fk_phone_user_id")
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_phone_user_id")
                        .from(UserPhone::Table, UserPhone::UserId)
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
            .drop_table(Table::drop().table(UserPhone::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserPhone {
    #[sea_orm(iden = "t_user_phone")]
    Table,
    Id,
    UserId,
    Phone,
    Desc,
    CreatedAt,
    UpdatedAt,
}
