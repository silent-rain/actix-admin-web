//! 部门表
//! User Entity: [`entity::prelude::PermDept`]

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
                    .table(PermDept::Table)
                    .comment("部门表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermDept::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("部门ID"),
                    )
                    .col(
                        ColumnDef::new(PermDept::Pid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("上级部门ID"),
                    )
                    .col(
                        ColumnDef::new(PermDept::Pids)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("所有上级部门ID, 用逗号分开"),
                    )
                    .col(
                        ColumnDef::new(PermDept::Name)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("部门名称"),
                    )
                    .col(
                        ColumnDef::new(PermDept::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(PermDept::Note)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(PermDept::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(PermDept::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermDept::UpdatedAt)
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
            .drop_table(Table::drop().table(PermDept::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermDept {
    #[sea_orm(iden = "t_perm_dept")]
    Table,
    Id,
    Pid,
    Pids,
    Name,
    Sort,
    Note,
    Status,
    CreatedAt,
    UpdatedAt,
}
