//! 验证码表

use sea_orm::{
    prelude::{BlobSize, DateTimeLocal},
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "sys_captcha")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub captcha_id: String,
    pub captcha: String,
    #[sea_orm(column_type = "Binary(BlobSize::Long)")]
    pub base_img: Vec<u8>,
    pub expire: u32,
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
