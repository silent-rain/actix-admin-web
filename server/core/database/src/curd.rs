//! 通用 CURD

use crate::{DbRepo, Pagination};

use async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult,
    IntoActiveModel, Iterable, PaginatorTrait, PrimaryKeyToColumn, PrimaryKeyTrait, QueryFilter,
    QuerySelect,
};

#[async_trait]
pub trait Curd<E>: Sync
where
    E: EntityTrait,
{
    type Db: DbRepo;

    fn db(&self) -> &Self::Db;

    async fn all(&self) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        E::Model: FromQueryResult,
    {
        let result = E::find().all(self.db().rdb()).await?;
        let total = result.len() as u64;
        Ok((result, total))
    }

    /// 获取数据列表
    async fn list(&self, page: Pagination) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        E::Model: FromQueryResult,
    {
        // 获取总页数
        // let pages: u64 = E::find()
        //     .paginate(self.db().rdb(), page.page_size())
        //     .num_items()
        //     .await?;

        // 获取总数
        // let total: u64 = E::find().paginate(self.db().rdb(), 1).num_items().await?;
        let total: u64 = E::find().all(self.db().rdb()).await?.len() as u64;

        let results = E::find()
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db().rdb())
            .await?;
        Ok((results, total))
    }

    /// 获取数据列表，对选择操作的结果进行分页
    async fn _list_pages(&self, page: Pagination) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        E::Model: FromQueryResult,
        E: EntityTrait<Model = E>,
    {
        let paginator = E::find().paginate(self.db().rdb(), page.page_size());
        let total_pages = paginator.num_items().await?;

        paginator
            .fetch_page(page.page())
            .await
            .map(|results| (results, total_pages))
    }

    /// 根据主键获取详情信息
    async fn info(&self, id: i32) -> Result<Option<E::Model>, DbErr>
    where
        <E::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        E::find_by_id(id).one(self.db().rdb()).await
    }

    /// 插入数据
    /// 插入一个模型并返回一个新的 Model .其值是从数据库中检索的，因此将填充任何自动生成的字段;
    /// 按值进行填充, 默认值可能会产生问题;
    async fn _insert2<M, A>(
        &self,
        model: M,
    ) -> Result<<<A as ActiveModelTrait>::Entity as EntityTrait>::Model, DbErr>
    where
        M: Send,
        A: ActiveModelBehavior + Send + From<M>,
        <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    {
        let active_model: A = model.into();
        active_model.insert(self.db().wdb()).await
    }

    /// 插入数据
    /// 插入一个活动模型并返回一个新的 Model .其值是从数据库中检索的，因此将填充任何自动生成的字段。
    async fn insert<A>(
        &self,
        bean: A,
    ) -> Result<<<A as ActiveModelTrait>::Entity as EntityTrait>::Model, DbErr>
    where
        A: ActiveModelTrait + ActiveModelBehavior + Send,
        <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    {
        bean.insert(self.db().wdb()).await
    }

    /// 更新数据
    /// 接受一个模型并尝试更新数据库中的记录;
    /// 按值进行填充，这是一个失败的更新示例;
    async fn _update2<M, A>(
        &self,
        model: M,
    ) -> Result<<<A as ActiveModelTrait>::Entity as EntityTrait>::Model, DbErr>
    where
        M: Send,
        A: ActiveModelBehavior + Send + From<M>,
        <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    {
        let active_model: A = model.into();
        active_model.update(self.db().wdb()).await
    }

    /// 更新数据
    /// 接受一个活动模型并尝试更新数据库中的记录。
    async fn update<A>(
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
        <E::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        let result = E::delete_by_id(id).exec(self.db().wdb()).await?;
        Ok(result.rows_affected)
    }

    /// 按主键批量删除
    async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr>
    where
        <E::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        let mut state = E::delete_many();
        for key in E::PrimaryKey::iter() {
            let col = key.into_column();
            state = state.filter(col.is_in(ids.clone()));
        }
        let result = state.exec(self.db().wdb()).await?;
        Ok(result.rows_affected)
    }
}
