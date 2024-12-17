//! OpenApi接口角色关系表
//! Entity: [`entity::permission::OpenapiRoleRel`]
use crate::{permission::openapi::Openapi, user::role::UserRole};

use sea_orm::{
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Index, Table},
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
                    .table(OpenapiRoleRel::Table)
                    .comment("OpenApi接口角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OpenapiRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(OpenapiRoleRel::ApiId)
                            .integer()
                            .not_null()
                            .comment("接口ID"),
                    )
                    .col(
                        ColumnDef::new(OpenapiRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(OpenapiRoleRel::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if !manager
            .has_index(OpenapiRoleRel::Table.to_string(), "uk_openapi_id_role_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(OpenapiRoleRel::Table)
                        .name("uk_openapi_id_role_id")
                        .unique()
                        .col(OpenapiRoleRel::ApiId)
                        .col(OpenapiRoleRel::RoleId)
                        .to_owned(),
                )
                .await?;
        }

        // Sqlite 不支持外键
        if manager.get_database_backend() == DatabaseBackend::Sqlite {
            return Ok(());
        }

        if !manager
            .has_index(
                OpenapiRoleRel::Table.to_string(),
                "fk_openapi_role_rel_openapi_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_openapi_role_rel_openapi_id")
                        .from(OpenapiRoleRel::Table, OpenapiRoleRel::ApiId)
                        .to(Openapi::Table, Openapi::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                OpenapiRoleRel::Table.to_string(),
                "fk_openapi_role_rel_role_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_openapi_role_rel_role_id")
                        .from(OpenapiRoleRel::Table, OpenapiRoleRel::RoleId)
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
            .drop_table(Table::drop().table(OpenapiRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum OpenapiRoleRel {
    #[sea_orm(iden = "t_perm_openapi_role_rel")]
    Table,
    Id,
    ApiId,
    RoleId,
    CreatedAt,
}
