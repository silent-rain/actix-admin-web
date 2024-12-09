//! 用户地理位置表
//! Entity: [`entity::user::Location`]

use sea_orm::{
    sea_query::{ColumnDef, Expr, Index, Table},
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
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
                    .table(Location::Table)
                    .comment("用户地理位置表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Location::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("地理位置ID"),
                    )
                    .col(
                        ColumnDef::new(Location::UserId)
                            .integer()
                            .unique_key()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(Location::Province)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("省份"),
                    )
                    .col(
                        ColumnDef::new(Location::City)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("城市"),
                    )
                    .col(
                        ColumnDef::new(Location::District)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("区/县"),
                    )
                    .col(
                        ColumnDef::new(Location::Address)
                            .string()
                            .string_len(255)
                            .not_null()
                            .comment("详细地址"),
                    )
                    .col(
                        ColumnDef::new(Location::PostalCode)
                            .string()
                            .string_len(255)
                            .null()
                            .default("")
                            .comment("邮政编码"),
                    )
                    .col(
                        ColumnDef::new(Location::Longitude)
                            .decimal()
                            .decimal_len(11, 8)
                            .not_null()
                            .default(0)
                            .comment("经度"),
                    )
                    .col(
                        ColumnDef::new(Location::Latitude)
                            .decimal()
                            .decimal_len(11, 8)
                            .not_null()
                            .default(0)
                            .comment("纬度"),
                    )
                    .col(
                        ColumnDef::new(Location::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(Location::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Location::UpdatedAt)
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

        if !manager
            .has_index(Location::Table.to_string(), "idx_user_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("idx_user_id")
                        .table(Location::Table)
                        .col(Location::UserId)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Location::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Location {
    #[sea_orm(iden = "t_user_location")]
    Table,
    Id,
    UserId,
    Province,
    City,
    District,
    Address,
    PostalCode,
    Longitude,
    Latitude,
    Desc,
    CreatedAt,
    UpdatedAt,
}
