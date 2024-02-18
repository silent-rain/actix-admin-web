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
                    .table(PermRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermRole::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(PermRole::Name)
                            .string()
                            .not_null()
                            .unique_key()
                            .comment("角色名称"),
                    )
                    .col(
                        ColumnDef::new(PermRole::Sort)
                            .integer()
                            .not_null()
                            .default(1)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(PermRole::Note)
                            .string()
                            .null()
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(PermRole::Status)
                            .tiny_integer()
                            .not_null()
                            .comment("状态;1:启用,2:禁用"),
                    )
                    .col(
                        ColumnDef::new(PermRole::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermRole::UpdatedAt)
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
            .drop_table(Table::drop().table(PermRole::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum PermRole {
    #[iden = "perm_user"]
    Table,
    Id,
    Name,
    Sort,
    Note,
    Status,
    CreatedAt,
    UpdatedAt,
}
