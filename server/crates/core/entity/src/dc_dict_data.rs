//! 字典数据表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 字典数据表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "dc_dict_data")]
pub struct Model {
    /// 字典项ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 字典维度ID
    pub dict_id: String,
    /// 字典项名称
    pub name: String,
    /// 字典项值
    pub value: String,
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
    #[sea_orm(
        belongs_to = "super::dc_dict_dim::Entity",
        from = "Column::DictId",
        to = "super::dc_dict_dim::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    DcDictDim,
}

impl Related<super::dc_dict_dim::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DcDictDim.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
