//! 注册

use database::DbRepo;
use entity::{perm_user, perm_user_email, perm_user_phone};

use nject::injectable;
use perm::enums::UserStatus;
use sea_orm::{ActiveModelTrait, DatabaseTransaction, DbErr, Set, TransactionTrait};

use crate::dto::register::RegisterReq;

/// 数据访问
#[injectable]
pub struct RegisterDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> RegisterDao<'a> {
    /// 添加用户
    pub async fn add_user(&self, req: RegisterReq) -> Result<perm_user::Model, DbErr> {
        let txn = self.db.wdb().begin().await?;

        // 添加用户
        let user = self.txn_add_user(&txn, req.clone()).await?;
        // 添加手机号
        if let Some(phone) = req.phone.clone() {
            self.txn_add_phone(&txn, user.id, phone).await?;
        }
        // 添加邮箱
        if let Some(email) = req.email {
            self.txn_add_email(&txn, user.id, email).await?;
        }

        txn.commit().await?;
        Ok(user)
    }

    /// 添加用户
    async fn txn_add_user(
        &self,
        txn: &DatabaseTransaction,
        req: RegisterReq,
    ) -> Result<perm_user::Model, DbErr> {
        let active_model = perm_user::ActiveModel {
            username: Set(req.username),
            real_name: Set(req.real_name),
            gender: Set(req.gender),
            age: Set(req.age),
            birthday: Set(req.birthday),
            avatar: Set(req.avatar),
            password: Set(req.password),
            status: Set(UserStatus::Enabled as i8),
            ..Default::default()
        };
        active_model.insert(txn).await
    }

    /// 添加手机号
    async fn txn_add_phone(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        phone: String,
    ) -> Result<perm_user_phone::Model, DbErr> {
        let active_model = perm_user_phone::ActiveModel {
            user_id: Set(user_id),
            phone: Set(phone),
            ..Default::default()
        };
        active_model.insert(txn).await
    }

    /// 添加邮箱
    async fn txn_add_email(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        email: String,
    ) -> Result<perm_user_email::Model, DbErr> {
        let active_model = perm_user_email::ActiveModel {
            user_id: Set(user_id),
            email: Set(email),
            ..Default::default()
        };
        active_model.insert(txn).await
    }
}
