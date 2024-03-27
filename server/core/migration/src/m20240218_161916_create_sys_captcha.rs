//! 验证码表

use entity::prelude::SysCaptcha;
use entity::sys_captcha::Column;

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
                    .table(SysCaptcha)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(Column::CaptchaId)
                            .string()
                            .string_len(40)
                            .not_null()
                            .comment("验证码ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Captcha)
                            .string()
                            .not_null()
                            .comment("验证码"),
                    )
                    .col(
                        ColumnDef::new(Column::BaseImg)
                            .blob(BlobSize::Long)
                            .not_null()
                            .comment("Base64图片"),
                    )
                    .col(
                        ColumnDef::new(Column::Expire)
                            .integer()
                            .not_null()
                            .comment("过期时间"),
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
            .drop_table(Table::drop().table(SysCaptcha).to_owned())
            .await
    }
}
