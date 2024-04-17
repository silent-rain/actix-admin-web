//! 验证码表
//! User Entity: [`entity::prelude::SysCaptcha`]
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
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(Column::CaptchaId)
                            .string()
                            .string_len(40)
                            .unique_key()
                            .not_null()
                            .comment("验证码ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Captcha)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("验证码"),
                    )
                    .col(
                        ColumnDef::new(Column::BaseImg)
                            .blob(BlobSize::Medium)
                            .not_null()
                            .comment("Base64图片"),
                    )
                    .col(
                        ColumnDef::new(Column::Expire)
                            .integer()
                            .not_null()
                            .comment("过期时间,秒"),
                    )
                    .col(
                        ColumnDef::new(Column::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:无效,1:有效"),
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
            .drop_table(Table::drop().table(SysCaptcha).to_owned())
            .await
    }
}
