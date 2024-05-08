//! 验证码表
//! Entity: [`entity::prelude::SysCaptcha`]

use sea_orm::{
    sea_query::{BlobSize, ColumnDef, Expr, Table},
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
                    .table(SysCaptcha::Table)
                    .comment("验证码表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysCaptcha::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(SysCaptcha::CaptchaId)
                            .string()
                            .string_len(40)
                            .unique_key()
                            .not_null()
                            .comment("验证码ID"),
                    )
                    .col(
                        ColumnDef::new(SysCaptcha::Captcha)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("验证码"),
                    )
                    .col(
                        ColumnDef::new(SysCaptcha::BaseImg)
                            .blob(BlobSize::Medium)
                            .not_null()
                            .comment("Base64图片"),
                    )
                    .col(
                        ColumnDef::new(SysCaptcha::Expire)
                            .integer()
                            .not_null()
                            .comment("过期时间,秒"),
                    )
                    .col(
                        ColumnDef::new(SysCaptcha::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态(0:无效,1:有效)"),
                    )
                    .col(
                        ColumnDef::new(SysCaptcha::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(SysCaptcha::UpdatedAt)
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
            .drop_table(Table::drop().table(SysCaptcha::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysCaptcha {
    #[sea_orm(iden = "t_sys_captcha")]
    Table,
    Id,
    CaptchaId,
    Captcha,
    BaseImg,
    Expire,
    Status,
    CreatedAt,
    UpdatedAt,
}
