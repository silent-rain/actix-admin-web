//! WEB日志表
//! Entity: [`entity::log::LogWeb`]

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
                    .table(LogWeb::Table)
                    .comment("WEB日志表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LogWeb::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("日志ID"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::Username)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::RequestId)
                            .string()
                            .string_len(32)
                            .null()
                            .comment("请求ID"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::OsType)
                            .tiny_integer()
                            .not_null()
                            .comment("终端类型(0:未知, 1:安卓, 2:IOS, 3:WEB)"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::ErrorType)
                            .tiny_integer()
                            .not_null()
                            .comment("错误类型(0:代码报错, 1:接口报错)"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::Level)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("日志级别"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::CallerLine)
                            .string()
                            .string_len(100)
                            .not_null()
                            .comment("日发生位置"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::Url)
                            .string()
                            .string_len(500)
                            .null()
                            .comment("请求地址"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::Msg)
                            .text()
                            .null()
                            .comment("日志消息"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::Stack)
                            .text()
                            .null()
                            .comment("堆栈信息"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(LogWeb::CreatedAt)
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
            .drop_table(Table::drop().table(LogWeb::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LogWeb {
    #[sea_orm(iden = "t_log_web")]
    Table,
    Id,
    UserId,
    Username,
    RequestId,
    OsType,
    ErrorType,
    Level,
    CallerLine,
    Url,
    Msg,
    Stack,
    Desc,
    CreatedAt,
}
