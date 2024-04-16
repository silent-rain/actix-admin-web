//! 配置表

use utils::list_tree::GenericTreeTrait;

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 配置表
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_sys_config")]
pub struct Model {
    /// 配置ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 父节点ID
    pub pid: Option<i32>,
    /// 配置名称
    pub name: String,
    /// 配置编码(英文)
    #[sea_orm(unique)]
    pub code: String,
    /// 配置值
    #[sea_orm(column_type = "Text", nullable)]
    pub value: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 配置描述
    pub desc: Option<String>,
    /// 状态, 0:停用,1:正常
    pub status: i8,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// 实现 `GenericTreeTrait` trait, 将列表数据转换为树结构
impl GenericTreeTrait for Model {
    fn id(&self) -> i32 {
        self.id
    }

    fn pid(&self) -> Option<i32> {
        self.pid
    }
}
