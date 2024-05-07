//! OpenApi接口与角色关系表
//! User Entity: [`entity::prelude::PermOpenApiRoleRel`]
use crate::{
    m20240218_145453_create_perm_open_api::PermOpenApi, m20240218_145453_create_perm_role::PermRole,
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
                    .table(PermOpenApiRoleRel::Table)
                    .comment("OpenApi接口与角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermOpenApiRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApiRoleRel::ApiId)
                            .integer()
                            .not_null()
                            .comment("接口ID"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApiRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(PermOpenApiRoleRel::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if !manager
            .has_index(PermOpenApiRoleRel::Table.to_string(), "uk_api_id_role_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(PermOpenApiRoleRel::Table)
                        .name("uk_api_id_role_id")
                        .unique()
                        .col(PermOpenApiRoleRel::ApiId)
                        .col(PermOpenApiRoleRel::RoleId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermOpenApiRoleRel::Table.to_string(),
                "fk_open_api_role_rel_api_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_open_api_role_rel_api_id")
                        .from(PermOpenApiRoleRel::Table, PermOpenApiRoleRel::ApiId)
                        .to(PermOpenApi::Table, PermOpenApi::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermOpenApiRoleRel::Table.to_string(),
                "fk_open_api_role_rel_role_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_open_api_role_rel_role_id")
                        .from(PermOpenApiRoleRel::Table, PermOpenApiRoleRel::RoleId)
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
            .drop_table(Table::drop().table(PermOpenApiRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermOpenApiRoleRel {
    #[sea_orm(iden = "t_perm_open_api_role_rel")]
    Table,
    Id,
    ApiId,
    RoleId,
    CreatedAt,
}
