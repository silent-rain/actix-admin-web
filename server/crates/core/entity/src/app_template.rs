//! 应用模板, 用于制作自定义服务模板

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "app_template")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub status: i8,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::perm_user::Entity",
        from = "Column::UserId",
        to = "super::perm_user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PermUser,
}

impl ActiveModelBehavior for ActiveModel {}