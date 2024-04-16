//! 系统日志表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 系统日志表
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_log_system")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: i32,
    /// 用户ID
    pub user_id: Option<i32>,
    /// 用户名称
    pub username: Option<String>,

    /// 日志记录器名称
    #[serde(default)]
    pub name: String,
    /// Parent Span Id
    #[serde(default)]
    pub parent_span_id: Option<u32>,
    /// Span Id
    #[serde(default)]
    pub span_id: Option<u32>,
    /// 模块路径
    #[serde(default)]
    pub module_path: Option<String>,
    /// 描述发生此元数据所描述的跨度或事件的系统部分
    #[serde(default)]
    pub target: String,
    /// 文件
    #[serde(default)]
    pub file: Option<String>,
    /// 报错行数
    #[serde(default)]
    pub line: Option<u32>,
    /// 日志级别
    #[serde(default)]
    pub level: String,
    /// 事件类型
    #[serde(default)]
    pub kind: String,
    /// 是否为事件
    #[serde(default)]
    pub is_event: bool,
    /// 是否为 span
    #[serde(default)]
    pub is_span: bool,
    /// 日志字段名称列表
    #[serde(default)]
    pub fields: Option<String>,
    /// fields 日志数据集
    #[sea_orm(column_type = "Text", nullable)]
    pub field_data: Option<String>,
    /// 日志信息
    #[sea_orm(column_type = "Text", nullable)]
    pub message: Option<String>,

    /// 业务码
    pub code: Option<i32>,
    /// 业务码信息
    pub code_msg: Option<String>,
    /// 堆栈信息
    #[sea_orm(column_type = "Text", nullable)]
    pub stack: Option<String>,

    /// 备注
    pub note: Option<String>,
    /// 创建时间
    pub created_at: Option<DateTimeLocal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
