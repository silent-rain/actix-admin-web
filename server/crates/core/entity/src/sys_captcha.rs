//! 验证码表

use sea_orm::{
    prelude::{BlobSize, DateTimeLocal},
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 验证码表
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_sys_captcha")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 验证码ID
    pub captcha_id: String,
    /// 验证码
    pub captcha: String,
    /// Base64图片
    #[sea_orm(column_type = "Binary(BlobSize::Long)")]
    pub base_img: Vec<u8>,
    /// 过期时间,秒
    pub expire: u32,
    /// 状态,0:无效,1:有效
    pub status: i8,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
