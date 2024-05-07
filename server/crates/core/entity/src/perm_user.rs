//! 用户表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 用户表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_user")]
pub struct Model {
    /// 用户ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户名称
    pub username: String,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别, 0:男,1:女,2:保密
    pub gender: i8,
    /// 年龄
    pub age: Option<i32>,
    /// 出生日期
    pub birthday: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 介绍
    pub intro: Option<String>,
    /// 备注
    pub note: Option<String>,
    /// 密码
    pub password: String,
    /// 状态,0:停用,1:正常
    pub status: i8,
    /// 部门ID
    pub dept_id: Option<i32>,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::perm_user_role_rel::Entity")]
    PermUserRoleRel,
    #[sea_orm(has_many = "super::perm_user_phone::Entity")]
    PermUserPhone,
    #[sea_orm(has_many = "super::perm_user_email::Entity")]
    PermUserEmail,
}

impl Related<super::perm_user_role_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermUserRoleRel.def()
    }
}

impl Related<super::perm_user_phone::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermUserPhone.def()
    }
}

impl Related<super::perm_user_email::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermUserEmail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// 枚举
pub mod enums {
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};
    use serde_repr::{Deserialize_repr, Serialize_repr};

    /// 用户状态
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Status {
        /// 停用
        Disabled = 0,
        /// 正常
        Enabled = 1,
    }

    /// 性别
    #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Gender {
        /// 男
        Male = 0,
        /// 女
        Female = 1,
        /// 保密
        Confidential = 2,
    }

    /// 注册用户类型
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum UserType {
        /// 手机号码
        #[serde(rename = "phone")]
        Phone,
        /// 邮箱
        #[serde(rename = "email")]
        Email,
    }

    impl Default for UserType {
        fn default() -> Self {
            Self::Phone
        }
    }

    /// 实现FromStr trait来定义如何从字符串解析为RegisterType
    impl FromStr for UserType {
        type Err = ();

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match input {
                "phone" => Ok(UserType::Phone),
                "email" => Ok(UserType::Email),
                _ => Err(()),
            }
        }
    }

    impl From<UserType> for String {
        fn from(value: UserType) -> Self {
            match value {
                UserType::Phone => "phone".to_owned(),
                UserType::Email => "email".to_owned(),
            }
        }
    }
}
