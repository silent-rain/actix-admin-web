//! 部门角色关系表
//! Entity: [`entity::organization::DepartmentRoleRel`]
use crate::{
    m20240218_145453_create_org_department::Department, m20240218_145453_create_user_role::UserRole,
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
                    .table(DepartmentRoleRel::Table)
                    .comment("部门角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DepartmentRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(DepartmentRoleRel::DepartmentId)
                            .integer()
                            .not_null()
                            .comment("部门ID"),
                    )
                    .col(
                        ColumnDef::new(DepartmentRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(DepartmentRoleRel::CreatedAt)
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
                DepartmentRoleRel::Table.to_string(),
                "uk_department_id_role_id",
            )
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(DepartmentRoleRel::Table)
                        .name("uk_department_id_role_id")
                        .unique()
                        .col(DepartmentRoleRel::DepartmentId)
                        .col(DepartmentRoleRel::RoleId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                DepartmentRoleRel::Table.to_string(),
                "fk_perm_department_role_rel_department_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_department_role_rel_department_id")
                        .from(DepartmentRoleRel::Table, DepartmentRoleRel::DepartmentId)
                        .to(Department::Table, Department::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                DepartmentRoleRel::Table.to_string(),
                "fk_perm_department_role_rel_role_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_department_role_rel_role_id")
                        .from(DepartmentRoleRel::Table, DepartmentRoleRel::RoleId)
                        .to(UserRole::Table, UserRole::Id)
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
            .drop_table(Table::drop().table(DepartmentRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DepartmentRoleRel {
    #[sea_orm(iden = "t_org_department_role_rel")]
    Table,
    Id,
    DepartmentId,
    RoleId,
    CreatedAt,
}
