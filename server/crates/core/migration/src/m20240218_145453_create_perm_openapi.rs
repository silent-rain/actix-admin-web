//! OpenApi接口表
//! Entity: [`entity::prelude::PermOpenapi`]

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
                    .table(PermOpenapi::Table)
                    .comment("OpenApi接口表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermOpenapi::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("接口ID"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapi::Pid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("父ID"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapi::Category)
                            .integer()
                            .not_null()
                            .comment("类别,0:目录,1:接口"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapi::Name)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("接口名称"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapi::Method)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("请求类型"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapi::Path)
                            .string()
                            .string_len(200)
                            .not_null()
                            .comment("资源路径"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapi::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapi::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapi::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapi::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapi::UpdatedAt)
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
            .drop_table(Table::drop().table(PermOpenapi::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermOpenapi {
    #[sea_orm(iden = "t_perm_openapi")]
    Table,
    Id,
    Pid,
    Category,
    Name,
    Method,
    Path,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
