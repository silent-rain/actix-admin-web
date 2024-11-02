//! 系统日志

use database::PoolTrait;
use entity::log_system;

use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, DbErr};

pub struct Dao<DB: PoolTrait> {
    db: DB,
}

impl<DB: PoolTrait> Dao<DB> {
    /// 创建对象
    pub fn new(db: DB) -> Self {
        Dao { db }
    }

    /// 添加详情信息
    pub async fn add(&self, data: log_system::Model) -> Result<log_system::Model, DbErr> {
        let mut active_model: log_system::ActiveModel = data.into();
        active_model.id = NotSet;
        active_model.insert(self.db.db()).await
    }
}
