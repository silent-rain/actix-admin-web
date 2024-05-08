//! 部门角色关系表
//! Entity: [`entity::prelude::PermDepartmentRoleRel`]
use crate::{
    m20240218_145453_create_perm_department::PermDepartment,
    m20240218_145453_create_perm_role::PermRole,
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
                    .table(PermDepartmentRoleRel::Table)
                    .comment("部门角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermDepartmentRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(PermDepartmentRoleRel::DepartmentId)
                            .integer()
                            .not_null()
                            .comment("部门ID"),
                    )
                    .col(
                        ColumnDef::new(PermDepartmentRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(PermDepartmentRoleRel::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if !manager
            .has_index(
                PermDepartmentRoleRel::Table.to_string(),
                "uk_department_id_role_id",
            )
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(PermDepartmentRoleRel::Table)
                        .name("uk_department_id_role_id")
                        .unique()
                        .col(PermDepartmentRoleRel::DepartmentId)
                        .col(PermDepartmentRoleRel::RoleId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermDepartmentRoleRel::Table.to_string(),
                "fk_perm_department_role_rel_department_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_department_role_rel_department_id")
                        .from(
                            PermDepartmentRoleRel::Table,
                            PermDepartmentRoleRel::DepartmentId,
                        )
                        .to(PermDepartment::Table, PermDepartment::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermDepartmentRoleRel::Table.to_string(),
                "fk_perm_department_role_rel_role_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_department_role_rel_role_id")
                        .from(PermDepartmentRoleRel::Table, PermDepartmentRoleRel::RoleId)
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
            .drop_table(Table::drop().table(PermDepartmentRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermDepartmentRoleRel {
    #[sea_orm(iden = "t_org_department_role_rel")]
    Table,
    Id,
    DepartmentId,
    RoleId,
    CreatedAt,
}
