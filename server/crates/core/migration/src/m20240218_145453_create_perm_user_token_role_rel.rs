//! 部门角色关联表
//! User Entity: [`entity::prelude::PermUserTokenRoleRel`]
use entity::{perm_user_token_role_rel::Column, prelude::PermUserTokenRoleRel};

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
                    .table(PermUserTokenRoleRel)
                    .comment("用户Token令牌与角色关联表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(Column::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(Column::TokenId)
                            .integer()
                            .not_null()
                            .comment("令牌ID"),
                    )
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(PermUserTokenRoleRel).to_owned())
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
