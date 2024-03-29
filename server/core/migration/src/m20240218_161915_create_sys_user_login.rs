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
                    .table(SysUserLogin::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysUserLogin::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(SysUserLogin::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(SysUserLogin::Username)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(SysUserLogin::RemoteAddr)
                            .string()
                            .string_len(64)
                            .comment("登录IP"),
                    )
                    .col(
                        ColumnDef::new(SysUserLogin::UserAgent)
                            .string()
                            .string_len(256)
                            .comment("用户代理"),
                    )
                    .col(
                        ColumnDef::new(SysUserLogin::Status)
                            .tiny_integer()
                            .not_null()
                            .comment("登录状态,0:禁用,1:启用"),
                    )
                    .col(
                        ColumnDef::new(SysUserLogin::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(SysUserLogin::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(SysUserLogin::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum SysUserLogin {
    #[iden = "sys_user_login"]
    Table,
    Id,
    UserId,
    Username,
    RemoteAddr,
    UserAgent,
    Status,
    CreatedAt,
    UpdatedAt,
}
