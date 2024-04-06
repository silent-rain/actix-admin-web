use sea_orm_migration::prelude::*;

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
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermUserRoleRel::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
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
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermUserRoleRel::UpdatedAt)
                            .date_time()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(PermUserRoleRel::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum PermUserRoleRel {
    #[iden = "perm_user_role_rel"]
    Table,
    Id,
    UserId,
    RoleId,
    CreatedAt,
    UpdatedAt,
}
