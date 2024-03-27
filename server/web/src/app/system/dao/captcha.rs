//! 验证码
use crate::app::system::dto::captcha::CaptchaListReq;

use database::{DbRepo, Pagination};
use entity::{prelude::SysCaptcha, sys_captcha};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[injectable]
pub struct CaptchaDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> CaptchaDao<'a> {
    /// 获取数据列表
    pub async fn list(&self, req: CaptchaListReq) -> Result<(Vec<sys_captcha::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let paginator = SysCaptcha::find()
            .order_by_asc(sys_captcha::Column::Id)
            .paginate(self.db.rdb(), page.page_size());

        let num_pages = paginator.num_items().await?;

        let results = paginator.fetch_page(page.page()).await?;

        Ok((results, num_pages))
    }

    /// 获取详情信息
    pub async fn info(&self, uuid: String) -> Result<Option<sys_captcha::Model>, DbErr> {
        SysCaptcha::find()
            .filter(sys_captcha::Column::CaptchaId.eq(uuid))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(&self, data: sys_captcha::Model) -> Result<sys_captcha::Model, DbErr> {
        let pear = sys_captcha::ActiveModel {
            captcha_id: Set(data.captcha_id),
            captcha: Set(data.captcha),
            base_img: Set(data.base_img),
            expire: Set(data.expire),
            ..Default::default() // all other attributes are `NotSet`
        };
        pear.insert(self.db.wdb()).await
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
