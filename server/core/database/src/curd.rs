//! 通用 CURD

use crate::{DbRepo, Pagination};

use async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult,
    IntoActiveModel, Iterable, PaginatorTrait, PrimaryKeyToColumn, PrimaryKeyTrait, QueryFilter,
    QuerySelect,
};

#[async_trait]
pub trait Curd<M>: Sync
where
    M: EntityTrait<Model = M> + FromQueryResult,
{
    type Db: DbRepo;

    fn db(&self) -> &Self::Db;

    /// 获取所有数据, 数据量过大时不推荐使用
    async fn all(&self) -> Result<(Vec<M>, u64), DbErr> {
        let result = M::find().all(self.db().rdb()).await?;
        let total = result.len() as u64;
        Ok((result, total))
    }

    /// 获取数据列表
    async fn list(&self, page: Pagination) -> Result<(Vec<M>, u64), DbErr> {
        // 获取总页数
        // let pages: u64 = M::find()
        //     .paginate(self.db().rdb(), page.page_size())
        //     .num_items()
        //     .await?;

        // 获取总数
        let total: u64 = M::find().paginate(self.db().rdb(), 1).num_items().await?;

        let results = M::find()
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db().rdb())
            .await?;
        Ok((results, total))
    }

    /// 获取数据列表，对选择操作的结果进行分页
    async fn list_pages(&self, page: Pagination) -> Result<(Vec<M>, u64), DbErr> {
        let paginator = M::find().paginate(self.db().rdb(), page.page_size());
        let num_pages = paginator.num_items().await?;

        paginator
            .fetch_page(page.page())
            .await
            .map(|results| (results, num_pages))
    }

    /// 根据主键获取详情信息
    async fn info(&self, id: i32) -> Result<Option<M>, DbErr>
    where
        <M::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        M::find_by_id(id).one(self.db().rdb()).await
    }

    /// 插入数据
    /// 插入一个模型并返回一个新的 Model .其值是从数据库中检索的，因此将填充任何自动生成的字段。
    async fn insert(
        &self,
        model: M,
    ) -> Result<<<M as ActiveModelTrait>::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior,
    {
        let pear = model.into_active_model();
        pear.insert(self.db().wdb()).await
    }

    /// 插入数据
    /// 插入一个活动模型并返回一个新的 Model .其值是从数据库中检索的，因此将填充任何自动生成的字段。
    async fn insert2<A>(
        &self,
        bean: A,
    ) -> Result<<<A as ActiveModelTrait>::Entity as EntityTrait>::Model, DbErr>
    where
        A: ActiveModelTrait + ActiveModelBehavior + std::marker::Send,
        <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    {
        bean.insert(self.db().wdb()).await
    }

    /// 更新数据
    /// 接受一个模型并尝试更新数据库中的记录。
    async fn update(
        &self,
        model: M,
    ) -> Result<<<M as ActiveModelTrait>::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior,
    {
        let pear = model.into_active_model();
        pear.update(self.db().wdb()).await
    }

    /// 更新数据
    /// 接受一个活动模型并尝试更新数据库中的记录。
    async fn update2<A>(
        &self,
        bean: A,
    ) -> Result<<<A as ActiveModelTrait>::Entity as EntityTrait>::Model, DbErr>
    where
        A: ActiveModelTrait + ActiveModelBehavior + std::marker::Send,
        <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    {
        bean.update(self.db().wdb()).await
    }

    /// 按主键删除
    async fn delete(&self, id: i32) -> Result<u64, DbErr>
    where
        <M::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        let result = M::delete_by_id(id).exec(self.db().wdb()).await?;
        Ok(result.rows_affected)
    }

    /// 按主键批量删除
    async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr>
    where
        <M::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        let mut state = M::delete_many();
        for key in M::PrimaryKey::iter() {
            let col = key.into_column();
            state = state.filter(col.is_in(ids.clone()));
        }
        let result = state.exec(self.db().wdb()).await?;
        Ok(result.rows_affected)
    }
}
