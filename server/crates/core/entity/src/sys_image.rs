//! 图片资源表

use sea_orm::{
    prelude::{BlobSize, DateTimeLocal},
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 图片资源表
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_sys_image")]
pub struct Model {
    /// 图片ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 图片名称
    pub name: String,
    /// HASH名称
    #[sea_orm(unique)]
    pub hash_name: String,
    /// Base64图片
    #[sea_orm(column_type = "Binary(BlobSize::Medium)")]
    pub base_img: Vec<u8>,
    /// 扩展类型:svg,png
    pub img_type: String,
    /// 图片大小
    pub img_size: i32,
    /// 备注
    pub note: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
