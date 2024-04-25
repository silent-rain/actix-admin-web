//! 部门角色关联表
//! User Entity: [`entity::prelude::PermDeptRoleRel`]
use crate::{
    m20240218_145452_create_perm_role::PermRole, m20240218_145453_create_perm_dept::PermDept,
};

use sea_orm::{
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Index, Table},
    DeriveIden, DeriveMigrationName, Iden,
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
                    .table(PermDeptRoleRel::Table)
                    .comment("部门角色关联表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermDeptRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(PermDeptRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(PermDeptRoleRel::DeptId)
                            .integer()
                            .not_null()
                            .comment("部门ID"),
                    )
                    .col(
                        ColumnDef::new(PermDeptRoleRel::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if !manager
            .has_index(PermDeptRoleRel::Table.to_string(), "uk_dept_id_role_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(PermDeptRoleRel::Table)
                        .name("uk_dept_id_role_id")
                        .unique()
                        .col(PermDeptRoleRel::DeptId)
                        .col(PermDeptRoleRel::RoleId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermDeptRoleRel::Table.to_string(),
                "fk_perm_dept_role_rel_dept_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_dept_role_rel_dept_id")
                        .from(PermDeptRoleRel::Table, PermDeptRoleRel::DeptId)
                        .to(PermDept::Table, PermDept::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermDeptRoleRel::Table.to_string(),
                "fk_perm_dept_role_rel_role_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_dept_role_rel_role_id")
                        .from(PermDeptRoleRel::Table, PermDeptRoleRel::RoleId)
                        .to(PermRole::Table, PermRole::Id)
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
            .drop_table(Table::drop().table(PermDeptRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermDeptRoleRel {
    #[sea_orm(iden = "t_perm_dept_role_rel")]
    Table,
    Id,
    DeptId,
    RoleId,
    CreatedAt,
}
