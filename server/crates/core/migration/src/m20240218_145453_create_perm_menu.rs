//! 菜单表
//! Entity: [`entity::prelude::PermMenu`]

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
                    .table(PermMenu::Table)
                    .comment("菜单表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermMenu::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("菜单ID"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::Pid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("父菜单ID"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::Title)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("菜单名称"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::IconClass)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("Icon图标类"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::MenuType)
                            .integer()
                            .not_null()
                            .comment("菜单类型(0:菜单,1:按钮)"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::OpenMethod)
                            .integer()
                            .not_null()
                            .comment("打开方式(0:组件,1:内链,2:外链)"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::Path)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("路由地址"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::ComponentPath)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("组件路径"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::RedirectTo)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("路由重定向"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::Link)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("链接地址:站内链地址/站外链地址"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::LinkTarget)
                            .string()
                            .string_len(20)
                            .null()
                            .default("_blank")
                            .comment("链接跳转方式,_blank/_self"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::IsHidden)
                            .integer()
                            .null()
                            .default(1)
                            .comment("是否隐藏(0:显示,1:隐藏)"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::IsAlwaysDisplayed)
                            .integer()
                            .null()
                            .default(1)
                            .comment("是否始终显示根菜单(0:隐藏,1:显示)"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::Permission)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("权限标识"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态(0:停用,1:正常)"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermMenu::UpdatedAt)
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
            .drop_table(Table::drop().table(PermMenu::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermMenu {
    #[sea_orm(iden = "t_perm_menu")]
    Table,
    Id,
    Pid,
    Title,
    IconClass,
    MenuType,
    OpenMethod,
    Path,
    ComponentPath,
    RedirectTo,
    Link,
    LinkTarget,
    IsHidden,
    IsAlwaysDisplayed,
    Permission,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
