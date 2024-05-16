//! 岗位表
//! Entity: [`entity::organization::Position`]
use crate::m20240218_145453_create_org_department::Department;

use sea_orm::{
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Table},
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
                    .table(Position::Table)
                    .comment("岗位表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Position::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("岗位ID"),
                    )
                    .col(
                        ColumnDef::new(Position::Name)
                            .string()
                            .string_len(20)
                            .unique_key()
                            .not_null()
                            .comment("岗位名称"),
                    )
                    .col(
                        ColumnDef::new(Position::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Position::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(Position::DepartmentId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属部门ID"),
                    )
                    .col(
                        ColumnDef::new(Position::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态(0:停用,1:正常)"),
                    )
                    .col(
                        ColumnDef::new(Position::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Position::UpdatedAt)
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

        // Sqlite 不支持外键
        if manager.get_database_backend() == DatabaseBackend::Sqlite {
            return Ok(());
        }

        if !manager
            .has_index(Position::Table.to_string(), "fk_org_position_department_id")
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_org_position_department_id")
                        .from(Position::Table, Position::DepartmentId)
                        .to(Department::Table, Department::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Position::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Position {
    #[sea_orm(iden = "t_org_position")]
    Table,
    Id,
    Name,
    Sort,
    Desc,
    DepartmentId,
    Status,
    CreatedAt,
    UpdatedAt,
}
