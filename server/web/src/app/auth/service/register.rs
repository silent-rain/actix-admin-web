//! 注册
use crate::app::{
    auth::dto::register::{EmailRegisterReq, PhoneRegisterReq},
    perm::UserDao,
};

use code::Error;
use entity::perm_user;
use sea_orm::Set;

use nject::injectable;
use tracing::error;

/// 服务
#[injectable]
pub struct RegisterService<'a> {
    user_dao: UserDao<'a>,
}

impl<'a> RegisterService<'a> {
    /// 注册手机用户
    pub async fn add_phone_user(&self, data: PhoneRegisterReq) -> Result<perm_user::Model, Error> {
        let data = perm_user::ActiveModel {
            username: Set(Some(data.username)),
            gender: Set(data.gender),
            age: Set(data.age),
            birthday: Set(data.birthday),
            avatar: Set(data.avatar),
            phone: Set(Some(data.phone)),
            password: Set(data.password),
            status: Set(1),
            ..Default::default()
        };

        let result = self.user_dao.add_user(data, vec![]).await.map_err(|err| {
            error!("添加数据失败, error: {err:#?}");
            Error::DBAddError
        })?;
        Ok(result)
    }

    /// 注册邮箱用户
    pub async fn add_email_user(&self, data: EmailRegisterReq) -> Result<perm_user::Model, Error> {
        let data = perm_user::ActiveModel {
            username: Set(Some(data.username)),
            gender: Set(data.gender),
            age: Set(data.age),
            birthday: Set(data.birthday),
            avatar: Set(data.avatar),
            email: Set(Some(data.email)),
            password: Set(data.password),
            status: Set(1),
            ..Default::default()
        };

        let result = self.user_dao.add_user(data, vec![]).await.map_err(|err| {
            error!("添加数据失败, error: {err:#?}");
            Error::DBAddError
        })?;
        Ok(result)
    }
}
