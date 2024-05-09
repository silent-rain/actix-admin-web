//! 图片验证码管理
use crate::dto::image_captcha::GetImageCaptchaListReq;

use database::{DbRepo, Pagination};
use entity::{prelude::SysImageCaptcha, sys_image_captcha};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct ImageCaptchaDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> ImageCaptchaDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetImageCaptchaListReq,
    ) -> Result<(Vec<sys_image_captcha::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = SysImageCaptcha::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(sys_image_captcha::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(sys_image_captcha::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(sys_image_captcha::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }
    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<sys_image_captcha::Model>, DbErr> {
        SysImageCaptcha::find()
            .filter(sys_image_captcha::Column::Id.eq(id))
            .one(self.db.rdb())
            .await
    }

    /// 通过captcha_id获取详情信息
    pub async fn info_by_captcha_id(
        &self,
        captcha_id: String,
    ) -> Result<Option<sys_image_captcha::Model>, DbErr> {
        SysImageCaptcha::find()
            .filter(sys_image_captcha::Column::CaptchaId.eq(captcha_id))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: sys_image_captcha::ActiveModel,
    ) -> Result<sys_image_captcha::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新信息
    pub async fn update(&self, data: sys_image_captcha::Model) -> Result<u64, DbErr> {
        // Into ActiveModel
        let pear: sys_image_captcha::ActiveModel = data.clone().into();

        let result = SysImageCaptcha::update_many()
            .set(pear)
            .filter(sys_image_captcha::Column::Id.eq(data.id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = sys_image_captcha::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.wdb()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = SysImageCaptcha::delete_by_id(id)
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }

    /// 按主键批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = SysImageCaptcha::delete_many()
            .filter(sys_image_captcha::Column::Id.is_in(ids))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}
