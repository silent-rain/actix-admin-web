//! 验证码
use crate::system::dto::captcha::GetCaptchaListReq;

use database::{DbRepo, Pagination};
use entity::{prelude::SysCaptcha, sys_captcha};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct CaptchaDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> CaptchaDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetCaptchaListReq,
    ) -> Result<(Vec<sys_captcha::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = SysCaptcha::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(sys_captcha::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(sys_captcha::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;

        let results = states
            .order_by_desc(sys_captcha::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }
    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<sys_captcha::Model>, DbErr> {
        SysCaptcha::find()
            .filter(sys_captcha::Column::Id.eq(id))
            .one(self.db.rdb())
            .await
    }

    /// 通过captcha_id获取详情信息
    pub async fn info_by_captcha_id(
        &self,
        captcha_id: String,
    ) -> Result<Option<sys_captcha::Model>, DbErr> {
        SysCaptcha::find()
            .filter(sys_captcha::Column::CaptchaId.eq(captcha_id))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: sys_captcha::ActiveModel,
    ) -> Result<sys_captcha::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新信息
    pub async fn update(&self, data: sys_captcha::Model) -> Result<u64, DbErr> {
        // Into ActiveModel
        let pear: sys_captcha::ActiveModel = data.clone().into();

        let result = SysCaptcha::update_many()
            .set(pear)
            .filter(sys_captcha::Column::Id.eq(data.id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = SysCaptcha::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }

    /// 按主键批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = SysCaptcha::delete_many()
            .filter(sys_captcha::Column::Id.is_in(ids))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}
