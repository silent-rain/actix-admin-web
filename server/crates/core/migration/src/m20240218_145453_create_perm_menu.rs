//! 菜单表
//! User Entity: [`entity::prelude::PermMenu`]
use entity::{perm_menu::Column, prelude::PermMenu};

use sea_orm_migration::{
    async_trait,
    sea_orm::{DatabaseBackend, DeriveMigrationName},
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
                    .table(PermMenu)
                    .comment("菜单表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("菜单ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Pid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("父菜单ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Title)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("菜单名称"),
                    )
                    .col(
                        ColumnDef::new(Column::Icon)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("Icon图标"),
                    )
                    .col(
                        ColumnDef::new(Column::ElIcon)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("Element-Ico图标"),
                    )
                    .col(
                        ColumnDef::new(Column::MenuType)
                            .integer()
                            .not_null()
                            .comment("菜单类型,0:菜单,1:按钮"),
                    )
                    .col(
                        ColumnDef::new(Column::OpenType)
                            .integer()
                            .not_null()
                            .comment("打开方式,0:组件,1:内链,2:外链"),
                    )
                    .col(
                        ColumnDef::new(Column::Path)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("路由地址"),
                    )
                    .col(
                        ColumnDef::new(Column::Component)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("组件路径"),
                    )
                    .col(
                        ColumnDef::new(Column::Redirect)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("路由重定向"),
                    )
                    .col(
                        ColumnDef::new(Column::Link)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("链接地址:站内链地址/站外链地址"),
                    )
                    .col(
                        ColumnDef::new(Column::LinkTarget)
                            .string()
                            .string_len(20)
                            .null()
                            .default("_blank")
                            .comment("链接跳转方式,_blank/_self"),
                    )
                    .col(
                        ColumnDef::new(Column::Hidden)
                            .integer()
                            .null()
                            .default(1)
                            .comment("是否隐藏,0:显示,1:隐藏"),
                    )
                    .col(
                        ColumnDef::new(Column::RootAlwaysShow)
                            .integer()
                            .null()
                            .default(1)
                            .comment("始终显示根菜单,0:显示,1:隐藏"),
                    )
                    .col(
                        ColumnDef::new(Column::Permission)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("权限标识"),
                    )
                    .col(
                        ColumnDef::new(Column::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Column::Note)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(Column::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Column::UpdatedAt)
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
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(PermMenu).to_owned())
            .await
    }
}
