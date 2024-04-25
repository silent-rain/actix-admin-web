//! 系统日志表
//! User Entity: [`entity::prelude::LogSystem`]

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
                    .table(LogSystem::Table)
                    .comment("系统日志表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LogSystem::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::UserId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("请求用户ID"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Username)
                            .string()
                            .string_len(32)
                            .null()
                            .default("")
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Name)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("日志记录器名称"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::SpanPid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("Parent Span Id"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::SpanId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("Span Id"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::ModulePath)
                            .string()
                            .string_len(100)
                            .null()
                            .default("")
                            .comment("模块路径"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Target)
                            .string()
                            .string_len(100)
                            .not_null()
                            .comment("描述发生此元数据所描述的跨度或事件的系统部分"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::File)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("文件"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Line)
                            .integer()
                            .null()
                            .default(0)
                            .comment("报错行数"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Level)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("日志级别"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Kind)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("事件类型"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::IsEvent)
                            .tiny_integer()
                            .not_null()
                            .comment("是否为事件"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::IsSpan)
                            .tiny_integer()
                            .not_null()
                            .comment("是否为 span"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Fields)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("日志字段名称列表"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::FieldData)
                            .text()
                            .null()
                            .comment("fields 日志数据集"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Message)
                            .text()
                            .null()
                            .comment("日志信息"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Code)
                            .integer()
                            .null()
                            .default(0)
                            .comment("业务误码"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::CodeMsg)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("业务误码信息"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Stack)
                            .text()
                            .null()
                            .comment("堆栈信息"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::Note)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(LogSystem::CreatedAt)
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
            .drop_table(Table::drop().table(LogSystem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LogSystem {
    #[sea_orm(iden = "t_log_system")]
    Table,
    Id,
    UserId,
    Username,
    Name,
    SpanPid,
    SpanId,
    ModulePath,
    Target,
    File,
    Line,
    Level,
    Kind,
    IsEvent,
    IsSpan,
    Fields,
    FieldData,
    Message,
    Code,
    CodeMsg,
    Stack,
    Note,
    CreatedAt,
}
