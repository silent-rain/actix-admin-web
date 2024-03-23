//! 系统日志

use database::DbRepo;
use entity::log::system;

use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, DbErr};

pub struct Dao<DB: DbRepo> {
    db: DB,
}

impl<DB: DbRepo> Dao<DB> {
    /// 创建对象
    pub fn new(db: DB) -> Self {
        Dao { db }
    }

    /// 添加详情信息
    pub async fn add(&self, data: system::Model) -> Result<system::Model, DbErr> {
        let mut active_model: system::ActiveModel = data.into();
        active_model.id = NotSet;
        active_model.insert(self.db.wdb()).await
    }
}
