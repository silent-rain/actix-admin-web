//! 图片验证码表
//! Entity: [`entity::prelude::SysImageCaptcha`]

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
                    .table(SysImageCaptcha::Table)
                    .comment("图片验证码表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysImageCaptcha::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(SysImageCaptcha::CaptchaId)
                            .string()
                            .string_len(40)
                            .unique_key()
                            .not_null()
                            .comment("验证码ID"),
                    )
                    .col(
                        ColumnDef::new(SysImageCaptcha::Captcha)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("验证码"),
                    )
                    .col(
                        ColumnDef::new(SysImageCaptcha::Data)
                            .blob()
                            .not_null()
                            .comment("图片数据, Base64编码"),
                    )
                    .col(
                        ColumnDef::new(SysImageCaptcha::Expire)
                            .integer()
                            .not_null()
                            .comment("过期时间,秒"),
                    )
                    .col(
                        ColumnDef::new(SysImageCaptcha::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态(0:失效,1:有效)"),
                    )
                    .col(
                        ColumnDef::new(SysImageCaptcha::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(SysImageCaptcha::UpdatedAt)
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
            .drop_table(Table::drop().table(SysImageCaptcha::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysImageCaptcha {
    #[sea_orm(iden = "t_sys_image_captcha")]
    Table,
    Id,
    CaptchaId,
    Captcha,
    Data,
    Expire,
    Status,
    CreatedAt,
    UpdatedAt,
}
