//! 角色表
//! User Entity: [`entity::prelude::PermRole`]
use entity::{perm_role::Column, prelude::PermRole};

use sea_orm_migration::{
    async_trait,
    sea_orm::DeriveMigrationName,
    sea_query::{ColumnDef, Table},
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
                    .table(PermRole)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Name)
                            .string()
                            .string_len(20)
                            .not_null()
                            .unique_key()
                            .comment("角色名称"),
                    )
                    .col(
                        ColumnDef::new(Column::Sort)
                            .integer()
                            .not_null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Column::Note)
                            .string()
                            .string_len(200)
                            .null()
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(Column::Status)
                            .tiny_integer()
                            .not_null()
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(Column::Creator)
                            .integer()
                            .null()
                            .comment("创建者"),
                    )
                    .col(
                        ColumnDef::new(Column::Updater)
                            .integer()
                            .null()
                            .comment("更新者"),
                    )
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(PermRole).to_owned())
            .await
    }
}
