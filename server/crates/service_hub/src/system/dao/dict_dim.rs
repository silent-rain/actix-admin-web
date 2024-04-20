//! 字典维度管理
use crate::system::dto::dict_dim::GetDictDimListReq;

use database::{DbRepo, Pagination};
use entity::{prelude::SysDictDim, sys_dict_dim};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct DictDimDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> DictDimDao<'a> {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<sys_dict_dim::Model>, u64), DbErr> {
        let results = SysDictDim::find()
            .order_by_asc(sys_dict_dim::Column::Id)
            .all(self.db.rdb())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetDictDimListReq,
    ) -> Result<(Vec<sys_dict_dim::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = SysDictDim::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(sys_dict_dim::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(sys_dict_dim::Column::CreatedAt.lt(v))
            })
            .apply_if(req.name, |query, v| {
                query.filter(sys_dict_dim::Column::Name.like(format!("{v}%")))
            })
            .apply_if(req.code, |query, v| {
                query.filter(sys_dict_dim::Column::Code.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(sys_dict_dim::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<sys_dict_dim::Model>, DbErr> {
        SysDictDim::find_by_id(id).one(self.db.rdb()).await
    }

    /// 通过名称获取详情信息
    pub async fn info_by_name(&self, name: String) -> Result<Option<sys_dict_dim::Model>, DbErr> {
        SysDictDim::find()
            .filter(sys_dict_dim::Column::Name.eq(name))
            .one(self.db.rdb())
            .await
    }

    /// 通过编码获取详情信息
    pub async fn info_by_code(&self, code: String) -> Result<Option<sys_dict_dim::Model>, DbErr> {
        SysDictDim::find()
            .filter(sys_dict_dim::Column::Code.eq(code))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: sys_dict_dim::ActiveModel,
    ) -> Result<sys_dict_dim::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: sys_dict_dim::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = SysDictDim::update_many()
            .set(active_model)
            .filter(sys_dict_dim::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = sys_dict_dim::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.wdb()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = SysDictDim::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
