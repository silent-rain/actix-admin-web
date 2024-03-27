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
                    .table(LogSystem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LogSystem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LogSystem::UserId).integer().null())
                    .col(ColumnDef::new(LogSystem::Nickname).string().null())
                    .col(ColumnDef::new(LogSystem::ParentSpanId).integer().null())
                    .col(ColumnDef::new(LogSystem::SpanId).integer().null())
                    .col(ColumnDef::new(LogSystem::Name).string().not_null())
                    .col(ColumnDef::new(LogSystem::ModulePath).string().null())
                    .col(ColumnDef::new(LogSystem::Target).string().not_null())
                    .col(ColumnDef::new(LogSystem::File).string().null())
                    .col(ColumnDef::new(LogSystem::Line).integer().null())
                    .col(ColumnDef::new(LogSystem::Level).string().not_null())
                    .col(ColumnDef::new(LogSystem::Kind).string().not_null())
                    .col(ColumnDef::new(LogSystem::IsEvent).tiny_integer().not_null())
                    .col(ColumnDef::new(LogSystem::IsSpan).tiny_integer().not_null())
                    .col(ColumnDef::new(LogSystem::Fields).string().null())
                    .col(ColumnDef::new(LogSystem::FieldData).string().null())
                    .col(ColumnDef::new(LogSystem::Message).string().null())
                    .col(ColumnDef::new(LogSystem::Code).integer().null())
                    .col(ColumnDef::new(LogSystem::CodeMsg).string().null())
                    .col(ColumnDef::new(LogSystem::Stack).string().null())
                    .col(ColumnDef::new(LogSystem::Note).string().null())
                    .col(ColumnDef::new(LogSystem::CreatedAt).date_time().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(LogSystem::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum LogSystem {
    #[iden = "log_system"]
    Table,
    Id,
    UserId,
    Nickname,
    Name,
    ParentSpanId,
    SpanId,
    ModulePath,
    Target,
    File,
    Line,
    Level,
    Kind,
    IsEvent,
    IsSpan,
    Fields,
    FieldData,
    Message,
    Code,
    CodeMsg,
    Stack,
    Note,
    CreatedAt,
}
