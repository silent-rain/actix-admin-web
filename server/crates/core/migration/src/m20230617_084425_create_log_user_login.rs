//! 用户登录日志表
//! User Entity: [`entity::prelude::LogUserLogin`]
use entity::{log_user_login::Column, prelude::LogUserLogin};

use sea_orm_migration::{
    async_trait,
    sea_orm::{DatabaseBackend, DeriveMigrationName},
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
                    .table(LogUserLogin)
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
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Username)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(Column::Token)
                            .string()
                            .string_len(250)
                            .not_null()
                            .comment("登陆令牌"),
                    )
                    .col(
                        ColumnDef::new(Column::RemoteAddr)
                            .string()
                            .string_len(64)
                            .null()
                            .default("")
                            .comment("登录IP"),
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
                        ColumnDef::new(Column::Device)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("设备"),
                    )
                    .col(
                        ColumnDef::new(Column::System)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("系统"),
                    )
                    .col(
                        ColumnDef::new(Column::Browser)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("浏览器"),
                    )
                    .col(
                        ColumnDef::new(Column::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("登录状态,0:失败,1:成功"),
                    )
                    .col(
                        ColumnDef::new(Column::Disabled)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("禁用状态,0:未禁用,1:禁用"),
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
            .drop_table(Table::drop().table(LogUserLogin).to_owned())
            .await
    }
}
