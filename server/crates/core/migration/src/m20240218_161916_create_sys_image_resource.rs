//! 图片资源表
//! Entity: [`entity::prelude::SysImage`]

use sea_orm::{
    sea_query::{ColumnDef, Expr, Table},
    DeriveIden, DeriveMigrationName,
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
                    .table(SysImage::Table)
                    .comment("图片资源表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysImage::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("图片ID"),
                    )
                    .col(
                        ColumnDef::new(SysImage::Name)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("图片名称"),
                    )
                    .col(
                        ColumnDef::new(SysImage::Hash)
                            .string()
                            .string_len(32)
                            .unique_key()
                            .not_null()
                            .comment("图片HASH值"),
                    )
                    .col(
                        ColumnDef::new(SysImage::Data)
                            .blob()
                            .not_null()
                            .comment("图片数据, Base64编码"),
                    )
                    .col(
                        ColumnDef::new(SysImage::Extension)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("图片文件扩展名, 如svg, png"),
                    )
                    .col(
                        ColumnDef::new(SysImage::Size)
                            .integer()
                            .not_null()
                            .comment("图片文件大小，单位为字节"),
                    )
                    .col(
                        ColumnDef::new(SysImage::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(SysImage::CreatedAt)
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
            .drop_table(Table::drop().table(SysImage::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysImage {
    #[sea_orm(iden = "t_sys_image_resource")]
    Table,
    Id,
    Name,
    Hash,
    Data,
    Extension,
    Size,
    Desc,
    CreatedAt,
}
