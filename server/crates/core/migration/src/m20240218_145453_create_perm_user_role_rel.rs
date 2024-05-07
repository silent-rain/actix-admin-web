//! 用户角色关系表
//! User Entity: [`entity::prelude::PermUserRoleRel`]

use crate::{
    m20240218_145453_create_perm_role::PermRole, m20240218_145453_create_perm_user::PermUser,
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
                    .table(PermUserRoleRel::Table)
                    .comment("用户角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermUserRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(PermUserRoleRel::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(PermUserRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(PermUserRoleRel::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if !manager
            .has_index(PermUserRoleRel::Table.to_string(), "uk_user_id_role_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(PermUserRoleRel::Table)
                        .name("uk_user_id_role_id")
                        .unique()
                        .col(PermUserRoleRel::UserId)
                        .col(PermUserRoleRel::RoleId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermUserRoleRel::Table.to_string(),
                "fk_perm_user_role_rel_user_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_user_role_rel_user_id")
                        .from(PermUserRoleRel::Table, PermUserRoleRel::UserId)
                        .to(PermUser::Table, PermUser::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermUserRoleRel::Table.to_string(),
                "fk_perm_user_role_rel_role_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_user_role_rel_role_id")
                        .from(PermUserRoleRel::Table, PermUserRoleRel::RoleId)
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
            .drop_table(Table::drop().table(PermUserRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermUserRoleRel {
    #[sea_orm(iden = "t_perm_user_role_rel")]
    Table,
    Id,
    UserId,
    RoleId,
    CreatedAt,
}
