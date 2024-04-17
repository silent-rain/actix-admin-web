//! OpenApi接口表
//! User Entity: [`entity::prelude::OpenApi`]
use entity::{open_api::Column, prelude::OpenApi};

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
                    .table(OpenApi)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("接口ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Pid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("父ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Category)
                            .integer()
                            .not_null()
                            .comment("类别,0:目录,1:接口"),
                    )
                    .col(
                        ColumnDef::new(Column::Name)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("接口名称"),
                    )
                    .col(
                        ColumnDef::new(Column::Method)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("请求类型"),
                    )
                    .col(
                        ColumnDef::new(Column::Path)
                            .string()
                            .string_len(200)
                            .not_null()
                            .comment("资源路径"),
                    )
                    .col(
                        ColumnDef::new(Column::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Column::Note)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(Column::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
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
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(OpenApi).to_owned())
            .await
    }
}
