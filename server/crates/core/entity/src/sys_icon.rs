//! ICON图标表

use sea_orm::{
    prelude::{BlobSize, DateTimeLocal},
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 验证码表
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "sys_icon")]
pub struct Model {
    /// 图标ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 图标名称
    #[sea_orm(unique)]
    pub name: String,
    /// Base64图片
    #[sea_orm(column_type = "Binary(BlobSize::Long)")]
    pub base_img: String,
    /// 图标类型,1:element,2:custom
    pub category: i8,
    /// 备注
    pub note: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
