//! 用户Token令牌与角色关系表
//! User Entity: [`entity::prelude::PermUserTokenRoleRel`]
use crate::{
    m20240218_145453_create_perm_role::PermRole,
    m20240218_145453_create_perm_user_token::PermUserToken,
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
                    .table(PermUserTokenRoleRel::Table)
                    .comment("用户Token令牌与角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermUserTokenRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(PermUserTokenRoleRel::TokenId)
                            .integer()
                            .not_null()
                            .comment("令牌ID"),
                    )
                    .col(
                        ColumnDef::new(PermUserTokenRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(PermUserTokenRoleRel::CreatedAt)
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
                PermUserTokenRoleRel::Table.to_string(),
                "uk_token_id_role_id",
            )
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(PermUserTokenRoleRel::Table)
                        .name("uk_token_id_role_id")
                        .unique()
                        .col(PermUserTokenRoleRel::TokenId)
                        .col(PermUserTokenRoleRel::RoleId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermUserTokenRoleRel::Table.to_string(),
                "fk_perm_user_token_role_rel_token_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_user_token_role_rel_token_id")
                        .from(PermUserTokenRoleRel::Table, PermUserTokenRoleRel::TokenId)
                        .to(PermUserToken::Table, PermUserToken::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                PermUserTokenRoleRel::Table.to_string(),
                "fk_perm_user_token_role_rel_role_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_user_token_role_rel_role_id")
                        .from(PermUserTokenRoleRel::Table, PermUserTokenRoleRel::RoleId)
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
            .drop_table(Table::drop().table(PermUserTokenRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermUserTokenRoleRel {
    #[sea_orm(iden = "t_perm_user_token_role_rel")]
    Table,
    Id,
    TokenId,
    RoleId,
    CreatedAt,
}
