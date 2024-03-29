//! 应用模板, 用于制作自定义服务模板

use sea_orm::{
    prelude::DateTime, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "app_template")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: String,
    pub status: i8,
    pub created_at: DateTime,
    pub updated_at: DateTime,
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
