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
                    .table(PermUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Username)
                            .string()
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(ColumnDef::new(PermUser::Nickname).string().comment("昵称"))
                    .col(
                        ColumnDef::new(PermUser::Gender)
                            .tiny_integer()
                            .not_null()
                            .comment("性别;1:男,2:女,3:保密"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Age)
                            .integer()
                            .null()
                            .comment("年龄"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Birthday)
                            .string()
                            .null()
                            .comment("出生日期"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Avatar)
                            .string()
                            .null()
                            .comment("头像地址"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Phone)
                            .string()
                            .null()
                            .comment("电话号码"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Email)
                            .string()
                            .null()
                            .comment("邮箱"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Intro)
                            .string()
                            .null()
                            .comment("个人介绍"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Note)
                            .string()
                            .null()
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Password)
                            .string()
                            .null()
                            .comment("hash 密码"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Status)
                            .tiny_integer()
                            .null()
                            .comment("状态;1:启用,2:禁用"),
                    )
                    .col(
                        ColumnDef::new(PermUser::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermUser::UpdatedAt)
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
            .drop_table(Table::drop().table(PermUser::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum PermUser {
    #[iden = "perm_user"]
    Table,
    Id,
    Username,
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
    Status,
    CreatedAt,
    UpdatedAt,
}
