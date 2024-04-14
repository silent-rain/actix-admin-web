//! 字典数据表
//! User Entity: [`entity::prelude::SysDictData`]
use entity::{prelude::SysDictData, sys_dict_data::Column};

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
                    .table(SysDictData)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("字典项ID"),
                    )
                    .col(
                        ColumnDef::new(Column::DimId)
                            .integer()
                            .not_null()
                            .comment("字典维度ID"),
                    )
                    .col(
                        ColumnDef::new(Column::DimCode)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("字典维度编码"),
                    )
                    .col(
                        ColumnDef::new(Column::Lable)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("字典标签"),
                    )
                    .col(
                        ColumnDef::new(Column::Value)
                            .text()
                            .not_null()
                            .comment("字典键值"),
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
            .drop_table(Table::drop().table(SysDictData).to_owned())
            .await
    }
}
