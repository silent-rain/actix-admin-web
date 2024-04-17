//! ICON图片表
//! User Entity: [`entity::prelude::SysIcon`]
use entity::{prelude::SysIcon, sys_icon::Column};

use sea_orm_migration::{
    async_trait,
    sea_orm::DeriveMigrationName,
    sea_query::{BlobSize, ColumnDef, Expr, Table},
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
                    .table(SysIcon)
                    .comment("ICON图片表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("图片ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Name)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("图片名称"),
                    )
                    .col(
                        ColumnDef::new(Column::HashName)
                            .string()
                            .string_len(32)
                            .unique_key()
                            .not_null()
                            .comment("HASH名称"),
                    )
                    .col(
                        ColumnDef::new(Column::BaseImg)
                            .blob(BlobSize::Medium)
                            .not_null()
                            .comment("Base64图片"),
                    )
                    .col(
                        ColumnDef::new(Column::IconType)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("扩展类型,svg,png"),
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
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(SysIcon).to_owned())
            .await
    }
}
