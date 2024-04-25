//! OpenApi接口表
//! User Entity: [`entity::prelude::PermOpenApi`]

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
                    .table(PermOpenApi::Table)
                    .comment("OpenApi接口表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermOpenApi::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("接口ID"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApi::Pid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("父ID"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApi::Category)
                            .integer()
                            .not_null()
                            .comment("类别,0:目录,1:接口"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApi::Name)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("接口名称"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApi::Method)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("请求类型"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApi::Path)
                            .string()
                            .string_len(200)
                            .not_null()
                            .comment("资源路径"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApi::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApi::Note)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApi::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApi::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApi::UpdatedAt)
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
            .drop_table(Table::drop().table(PermOpenApi::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermOpenApi {
    #[sea_orm(iden = "t_perm_open_api")]
    Table,
    Id,
    Pid,
    Category,
    Name,
    Method,
    Path,
    Sort,
    Note,
    Status,
    CreatedAt,
    UpdatedAt,
}
