//! 用户会员等级表
//! Entity: [`entity::user::MemberLevel`]

use sea_orm::{
    sea_query::{ColumnDef, Expr, Table},
    DatabaseBackend, DeriveIden, DeriveMigrationName,
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
                    .table(MemberLevel::Table)
                    .comment("用户会员等级表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MemberLevel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("会员等级ID"),
                    )
                    .col(
                        ColumnDef::new(MemberLevel::Name)
                            .string()
                            .string_len(20)
                            .unique_key()
                            .not_null()
                            .comment("会员等级名称"),
                    )
                    .col(
                        ColumnDef::new(MemberLevel::Level)
                            .integer()
                            .unsigned()
                            .unique_key()
                            .not_null()
                            .comment("会员等级"),
                    )
                    .col(
                        ColumnDef::new(MemberLevel::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(MemberLevel::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("会员描述"),
                    )
                    .col(
                        ColumnDef::new(MemberLevel::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态(0:停用,1:正常)"),
                    )
                    .col(
                        ColumnDef::new(MemberLevel::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(MemberLevel::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra({
                                match manager.get_database_backend() {
                                    DatabaseBackend::Sqlite => "DEFAULT CURRENT_TIMESTAMP",
                                    _ => "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP",
                                }
                            })
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(MemberLevel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MemberLevel {
    #[sea_orm(iden = "t_user_member_level")]
    Table,
    Id,
    Name,
    Level,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
