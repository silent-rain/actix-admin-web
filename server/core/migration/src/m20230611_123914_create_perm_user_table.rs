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
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Realname).string().not_null())
                    .col(ColumnDef::new(User::Nickname).string().not_null())
                    .col(ColumnDef::new(User::Gender).tiny_integer().not_null())
                    .col(ColumnDef::new(User::Age).integer().null())
                    .col(ColumnDef::new(User::Birthday).string().null())
                    .col(ColumnDef::new(User::Avatar).string().null())
                    .col(ColumnDef::new(User::Phone).string().null())
                    .col(ColumnDef::new(User::Email).string().null())
                    .col(ColumnDef::new(User::Intro).string().null())
                    .col(ColumnDef::new(User::Note).string().null())
                    .col(ColumnDef::new(User::Password).string().null())
                    .col(ColumnDef::new(User::Sort).integer().null())
                    .col(ColumnDef::new(User::Status).tiny_integer().null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .date_time()
                            .null()
                            .comment("创建时间"),
                    )
                    .col(ColumnDef::new(User::UpdatedAt).date_time().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    #[iden = "perm_user"]
    Table,
    Id,
    Realname,
    Nickname,
    Gender,
    Age,
    Birthday,
    Avatar,
    Phone,
    Email,
    Intro,
    Note,
    Password,
    Sort,
    Status,
    CreatedAt,
    UpdatedAt,
}
