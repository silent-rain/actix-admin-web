//! 注册

use std::sync::Arc;

use database::PoolTrait;
use entity::user::{email, phone, user_base};

use nject::injectable;
use sea_orm::{ActiveModelTrait, DatabaseTransaction, DbErr, Set, TransactionTrait};

use crate::dto::register::RegisterReq;

/// 数据访问
#[injectable]
pub struct RegisterDao {
    db: Arc<dyn PoolTrait>,
}

impl RegisterDao {
    /// 添加用户
    pub async fn add_user(&self, req: RegisterReq) -> Result<user_base::Model, DbErr> {
        let txn = self.db.db().begin().await?;

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
    ) -> Result<user_base::Model, DbErr> {
        let active_model = user_base::ActiveModel {
            username: Set(req.username),
            real_name: Set(req.real_name),
            gender: Set(req.gender),
            age: Set(req.age),
            date_birth: Set(req.date_birth),
            avatar: Set(req.avatar),
            password: Set(req.password),
            status: Set(user_base::enums::Status::Enabled as i8),
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
    ) -> Result<phone::Model, DbErr> {
        let active_model = phone::ActiveModel {
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
    ) -> Result<email::Model, DbErr> {
        let active_model = email::ActiveModel {
            user_id: Set(user_id),
            email: Set(email),
            ..Default::default()
        };
        active_model.insert(txn).await
    }
}
