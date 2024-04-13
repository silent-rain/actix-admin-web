//! API操作日志表
//! User Entity: [`entity::prelude::LogApiOperation`]
use entity::{log_api_operation::Column, prelude::LogApiOperation};

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
                    .table(LogApiOperation)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(Column::UserId)
                            .integer()
                            .null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Username)
                            .string()
                            .string_len(32)
                            .null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(Column::RequestId)
                            .string()
                            .string_len(32)
                            .null()
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
                            .comment("请求IP"),
                    )
                    .col(
                        ColumnDef::new(Column::UserAgent)
                            .string()
                            .string_len(256)
                            .null()
                            .comment("用户代理"),
                    )
                    .col(
                        ColumnDef::new(Column::Cost)
                            .double()
                            .not_null()
                            .comment("耗时,纳秒"),
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
                            .string_len(255)
                            .null()
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .timestamp_with_time_zone()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
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
