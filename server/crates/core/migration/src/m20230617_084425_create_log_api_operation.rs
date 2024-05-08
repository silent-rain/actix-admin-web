//! API操作日志表
//! Entity: [`entity::prelude::LogApiOperation`]

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
                    .table(LogApiOperation::Table)
                    .comment("API操作日志表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LogApiOperation::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::UserId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::Username)
                            .string()
                            .string_len(32)
                            .null()
                            .default("")
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::RequestId)
                            .string()
                            .string_len(32)
                            .null()
                            .default("")
                            .comment("请求ID"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::StatusCode)
                            .integer()
                            .not_null()
                            .comment("请求状态码"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::Method)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("请求方法"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::Path)
                            .string()
                            .string_len(500)
                            .not_null()
                            .comment("请求地址路径"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::Query)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("请求参数"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::Body)
                            .text()
                            .null()
                            .comment("请求体/响应体"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::RemoteAddr)
                            .string()
                            .string_len(64)
                            .null()
                            .default("")
                            .comment("请求IP"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::UserAgent)
                            .string()
                            .string_len(256)
                            .null()
                            .default("")
                            .comment("用户代理"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::Cost)
                            .integer()
                            .unsigned()
                            .not_null()
                            .comment("耗时,毫秒"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::HttpType)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("请求类型:REQ/RSP"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(LogApiOperation::CreatedAt)
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
            .drop_table(Table::drop().table(LogApiOperation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LogApiOperation {
    #[sea_orm(iden = "t_log_api_operation")]
    Table,
    Id,
    UserId,
    Username,
    RequestId,
    StatusCode,
    Method,
    Path,
    Query,
    Body,
    RemoteAddr,
    UserAgent,
    Cost,
    HttpType,
    Desc,
    CreatedAt,
}
