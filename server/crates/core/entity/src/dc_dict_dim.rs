//! 字典维度表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 字典维度表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "dc_dict_dim")]
pub struct Model {
    /// 字典ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 字典名称
    #[sea_orm(unique)]
    pub name: String,
    /// 字典编码
    #[sea_orm(unique)]
    pub code: String,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    pub status: i8,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::dc_dict_data::Entity")]
    DcDictData,
}

impl Related<super::dc_dict_data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DcDictData.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
