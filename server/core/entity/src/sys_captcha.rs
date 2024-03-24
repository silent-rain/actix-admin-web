//! 验证码表

use sea_orm::{
    prelude::DateTime, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "sys_captcha")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub captcha: String,
    pub base_img: String,
    pub expire: i8,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
