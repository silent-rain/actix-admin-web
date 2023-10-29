//!通用 DAO 方法

use database::Pool;
use dto::pagination::Pagination;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DbErr, FromQueryResult, QuerySelect};
use sea_orm::{EntityTrait, PaginatorTrait};

/// 获取详情的通用特征
#[async_trait]
pub trait DaoInfo {
    /// 按主键获取详情信息
    async fn info<E>(db: &Pool, id: i32) -> Result<Option<E>, DbErr>
    where
        E: EntityTrait<Model = E> + sea_orm::FromQueryResult,
        <<E as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType:
            std::convert::From<i32>,
    {
        E::find_by_id(id).one(db.rdb()).await
    }
}

/// 删除数据的通用特征
#[async_trait]
pub trait DaoDelete {
    /// 按主键删除
    async fn delete<E>(db: &Pool, id: i32) -> Result<u64, DbErr>
    where
        E: EntityTrait<Model = E>,
        <<E as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType:
            std::convert::From<i32>,
    {
        let result = E::delete_by_id(id).exec(db.wdb()).await?;
        Ok(result.rows_affected)
    }
}

/// 添加数据的通用特征
#[async_trait]
pub trait DaoAdd {
    /// 插入数据
    /// 插入一个活动模型并返回一个新的 Model .其值是从数据库中检索的，因此将填充任何自动生成的字段。
    async fn add<A>(
        db: &Pool,
        bean: A,
    ) -> Result<<<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model, DbErr>
    where
        A: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
        <<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model:
            sea_orm::IntoActiveModel<A>,
    {
        bean.insert(db.wdb()).await
    }
}

/// 获取所有数据的通用特征
#[async_trait]
pub trait DaoAll {
    /// 获取所有数据, 一次返回所有的数据
    /// 默认顺序返回数据列表
    async fn all<E>(db: &Pool) -> Result<Vec<E>, DbErr>
    where
        E: EntityTrait<Model = E> + sea_orm::FromQueryResult,
    {
        let result = E::find().all(db.rdb()).await?;
        Ok(result)
    }
}

/// 获取分页数据的通用特征
#[async_trait]
pub trait DaoList {
    /// 获取分页数据列表
    async fn list<E, M>(db: &Pool, page: Pagination) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        E: EntityTrait<Model = M>,
        M: FromQueryResult + Sized + Send + Sync,
    {
        let total: u64 = E::find().paginate(db.rdb(), 1).num_items().await?;

        let results = E::find()
            .offset(page.offset())
            .limit(page.page_size())
            .all(db.rdb())
            .await?;
        Ok((results, total))
    }

    /// 获取分页数据列表及分页数
    async fn list_pages<E, M>(db: &Pool, page: Pagination) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        E: EntityTrait<Model = M>,
        M: FromQueryResult + Sized + Send + Sync,
    {
        let paginator = E::find().paginate(db.rdb(), page.page_size());
        let num_pages = paginator.num_items().await?;

        paginator
            .fetch_page(page.page())
            .await
            .map(|results| (results, num_pages))
    }
}
