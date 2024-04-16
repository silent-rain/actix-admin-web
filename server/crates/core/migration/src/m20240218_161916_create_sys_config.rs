//! 配置表
//! User Entity: [`entity::prelude::SysConfig`]
use entity::{prelude::SysConfig, sys_config::Column};

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
                    .table(SysConfig)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("配置ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Pid)
                            .integer()
                            .null()
                            .comment("父节点ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Name)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("配置名称"),
                    )
                    .col(
                        ColumnDef::new(Column::Code)
                            .string()
                            .string_len(64)
                            .not_null()
                            .unique_key()
                            .comment("配置编码(英文)"),
                    )
                    .col(
                        ColumnDef::new(Column::Value)
                            .text()
                            .null()
                            .comment("配置值"),
                    )
                    .col(
                        ColumnDef::new(Column::Sort)
                            .integer()
                            .unsigned()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Column::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .comment("配置描述"),
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
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SysConfig).to_owned())
            .await
    }
}
