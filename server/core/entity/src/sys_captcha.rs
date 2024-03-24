//! 验证码表

use sea_orm::{
    prelude::{BlobSize, DateTime},
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "sys_captcha")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub captcha: String,
    #[sea_orm(column_type = "Binary(BlobSize::Long)")]
    pub base_img: Vec<u8>,
    pub expire: i8,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
