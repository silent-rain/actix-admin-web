//! 菜单角色关联表
//! User Entity: [`entity::prelude::PermMenuRoleRel`]
use entity::{perm_menu_role_rel::Column, prelude::PermMenuRoleRel};

use sea_orm_migration::{
    async_trait,
    sea_orm::DeriveMigrationName,
    sea_query::{ColumnDef, Expr, Table},
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
                    .table(PermMenuRoleRel)
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
                        ColumnDef::new(Column::MenuId)
                            .integer()
                            .not_null()
                            .comment("菜单ID"),
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
            .drop_table(Table::drop().table(PermMenuRoleRel).to_owned())
            .await
    }
}