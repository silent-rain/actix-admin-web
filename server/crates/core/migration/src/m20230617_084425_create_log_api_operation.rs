//! API操作日志表
//! User Entity: [`entity::prelude::LogApiOperation`]
use entity::{log_api_operation::Column, prelude::LogApiOperation};

use sea_orm_migration::{
    async_trait,
    sea_orm::DeriveMigrationName,
    sea_query::{ColumnDef, Expr, Table},
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
                    .table(LogApiOperation)
                    .comment("API操作日志表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(Column::UserId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Username)
                            .string()
                            .string_len(32)
                            .null()
                            .default("")
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(Column::RequestId)
                            .string()
                            .string_len(32)
                            .null()
                            .default("")
                            .comment("请求ID"),
                    )
                    .col(
                        ColumnDef::new(Column::StatusCode)
                            .integer()
                            .not_null()
                            .comment("请求状态码"),
                    )
                    .col(
                        ColumnDef::new(Column::Method)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("请求方法"),
                    )
                    .col(
                        ColumnDef::new(Column::Path)
                            .string()
                            .string_len(500)
                            .not_null()
                            .comment("请求地址路径"),
                    )
                    .col(
                        ColumnDef::new(Column::Query)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("请求参数"),
                    )
                    .col(
                        ColumnDef::new(Column::Body)
                            .text()
                            .null()
                            .comment("请求体/响应体"),
                    )
                    .col(
                        ColumnDef::new(Column::RemoteAddr)
                            .string()
                            .string_len(64)
                            .null()
                            .default("")
                            .comment("请求IP"),
                    )
                    .col(
                        ColumnDef::new(Column::UserAgent)
                            .string()
                            .string_len(256)
                            .null()
                            .default("")
                            .comment("用户代理"),
                    )
                    .col(
                        ColumnDef::new(Column::Cost)
                            .decimal()
                            .decimal_len(10, 2)
                            .not_null()
                            .comment("耗时,毫秒"),
                    )
                    .col(
                        ColumnDef::new(Column::HttpType)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("请求类型:REQ/RSP"),
                    )
                    .col(
                        ColumnDef::new(Column::Note)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
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
            .drop_table(Table::drop().table(LogApiOperation).to_owned())
            .await
    }
}
