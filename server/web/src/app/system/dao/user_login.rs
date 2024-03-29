//! 登陆日志

use crate::app::system::dto::user_login::UserLoginListReq;

use database::{DbRepo, Pagination};
use entity::prelude::SysUserLogin;
use entity::sys_user_login;

use nject::injectable;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, PaginatorTrait, QueryOrder, QuerySelect, Set};

#[injectable]
pub struct UserLoginDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> UserLoginDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: UserLoginListReq,
    ) -> Result<(Vec<sys_user_login::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let total: u64 = SysUserLogin::find()
            .paginate(self.db.rdb(), 1)
            .num_items()
            .await?;

        let results = SysUserLogin::find()
            .offset(page.offset())
            .limit(page.page_size())
            .order_by_desc(sys_user_login::Column::Id)
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<sys_user_login::Model>, DbErr> {
        SysUserLogin::find_by_id(id).one(self.db.rdb()).await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: sys_user_login::ActiveModel,
    ) -> Result<sys_user_login::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 禁用登陆
    pub async fn disbale_status(&self, id: i32) -> Result<(), DbErr> {
        let active_model = sys_user_login::ActiveModel {
            id: Set(id),
            status: Set(0),
            ..Default::default()
        };
        let _ = active_model.update(self.db.wdb()).await?;
        Ok(())
    }
}
