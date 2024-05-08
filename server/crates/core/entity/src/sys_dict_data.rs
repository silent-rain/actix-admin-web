//! 字典数据表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 字典数据表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_sys_dict_data")]
pub struct Model {
    /// 字典项ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 字典维度ID
    pub dim_id: i32,
    /// 字典维度编码
    pub dim_code: String,
    /// 字典标签
    pub lable: String,
    /// 字典键值
    pub value: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
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
        belongs_to = "super::sys_dict_dim::Entity",
        from = "Column::DimId",
        to = "super::sys_dict_dim::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    DcDictDim,
}

impl Related<super::sys_dict_dim::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DcDictDim.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// 枚举
pub mod enums {
    use serde_repr::{Deserialize_repr, Serialize_repr};

    /// 字典数据状态
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Status {
        /// 停用
        Disabled = 0,
        /// 正常
        Enabled = 1,
    }
}
