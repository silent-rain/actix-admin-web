//! 字典数据表
//! Entity: [`entity::prelude::SysDictData`]
use crate::m20240218_161916_create_sys_dict_dim::SysDictDim;

use sea_orm::{
    sea_query::{ColumnDef, Expr, ForeignKey, Index, Table},
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
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
                    .table(SysDictData::Table)
                    .comment("字典数据表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysDictData::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("字典项ID"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::DimId)
                            .integer()
                            .not_null()
                            .comment("字典维度ID"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::DimCode)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("字典维度编码"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::Lable)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("字典标签"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::Value)
                            .text()
                            .not_null()
                            .comment("字典键值"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::UpdatedAt)
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
            .await?;

        if !manager
            .has_index(SysDictData::Table.to_string(), "idx_dim_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("idx_dim_id")
                        .table(SysDictData::Table)
                        .col(SysDictData::DimId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(SysDictData::Table.to_string(), "fk_sys_dict_data_dim_id")
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_t_sys_dict_data_dim_id")
                        .from(SysDictData::Table, SysDictData::DimId)
                        .to(SysDictDim::Table, SysDictDim::Id)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SysDictData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysDictData {
    #[sea_orm(iden = "t_sys_dict_data")]
    Table,
    Id,
    DimId,
    DimCode,
    Lable,
    Value,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
