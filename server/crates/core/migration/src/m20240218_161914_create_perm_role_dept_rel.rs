//! 角色部门关联表
//! User Entity: [`entity::prelude::PermRoleDeptRel`]
use entity::{perm_role_dept_rel::Column, prelude::PermRoleDeptRel};

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
                    .table(PermRoleDeptRel)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(Column::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(Column::DeptId)
                            .integer()
                            .not_null()
                            .comment("部门ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Creator)
                            .integer()
                            .null()
                            .comment("创建者"),
                    )
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(PermRoleDeptRel).to_owned())
            .await
    }
}
