//! OpenApi接口角色关系表
//! Entity: [`entity::prelude::PermOpenapiRoleRel`]
use crate::{
    m20240218_145453_create_perm_openapi::PermOpenapi, m20240218_145453_create_user_role::UserRole,
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
                    .table(PermOpenapiRoleRel::Table)
                    .comment("OpenApi接口角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermOpenapiRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapiRoleRel::ApiId)
                            .integer()
                            .not_null()
                            .comment("接口ID"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapiRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(PermOpenapiRoleRel::CreatedAt)
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
                PermOpenapiRoleRel::Table.to_string(),
                "uk_openapi_id_role_id",
            )
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(PermOpenapiRoleRel::Table)
                        .name("uk_openapi_id_role_id")
                        .unique()
                        .col(PermOpenapiRoleRel::ApiId)
                        .col(PermOpenapiRoleRel::RoleId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermOpenapiRoleRel::Table.to_string(),
                "fk_openapi_role_rel_openapi_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_openapi_role_rel_openapi_id")
                        .from(PermOpenapiRoleRel::Table, PermOpenapiRoleRel::ApiId)
                        .to(PermOpenapi::Table, PermOpenapi::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermOpenapiRoleRel::Table.to_string(),
                "fk_openapi_role_rel_role_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_openapi_role_rel_role_id")
                        .from(PermOpenapiRoleRel::Table, PermOpenapiRoleRel::RoleId)
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
            .drop_table(Table::drop().table(PermOpenapiRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermOpenapiRoleRel {
    #[sea_orm(iden = "t_perm_openapi_role_rel")]
    Table,
    Id,
    ApiId,
    RoleId,
    CreatedAt,
}
