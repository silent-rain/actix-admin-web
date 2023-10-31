//!通用 DAO 方法

use database::Pool;
use dto::pagination::Pagination;

use sea_orm::{ActiveModelTrait, DbErr, FromQueryResult, QuerySelect};
use sea_orm::{EntityTrait, PaginatorTrait};

pub struct Dao<'a> {
    db: &'a Pool,
}

impl<'a> Dao<'a> {
    /// 创建 DAO 对象
    pub fn new(db: &'a Pool) -> Dao<'a> {
        Dao { db }
    }

    /// 按主键获取详情信息
    pub async fn info<E>(&self, id: i32) -> Result<Option<E>, DbErr>
    where
        E: EntityTrait<Model = E>,
        <<E as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType:
            std::convert::From<i32>,
    {
        E::find_by_id(id).one(self.db.rdb()).await
    }

    /// 按主键删除
    pub async fn delete<E>(&self, id: i32) -> Result<u64, DbErr>
    where
        E: EntityTrait<Model = E>,
        <<E as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType:
            std::convert::From<i32>,
    {
        let result = E::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }

    /// 插入数据
    /// 插入一个活动模型并返回一个新的 Model .其值是从数据库中检索的，因此将填充任何自动生成的字段。
    pub async fn add<A>(
        &self,
        bean: A,
    ) -> Result<<<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model, DbErr>
    where
        A: ActiveModelTrait + sea_orm::ActiveModelBehavior + std::marker::Send,
        <<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model:
            sea_orm::IntoActiveModel<A>,
    {
        bean.insert(self.db.wdb()).await
    }

    /// 获取所有数据, 一次返回所有的数据
    /// 默认顺序返回数据列表
    pub async fn all<E>(&self) -> Result<Vec<E>, DbErr>
    where
        E: EntityTrait<Model = E>,
    {
        let result = E::find().all(self.db.rdb()).await?;
        Ok(result)
    }

    /// 获取分页数据列表
    pub async fn list<E, M>(&self, page: Pagination) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        E: EntityTrait<Model = M>,
        M: FromQueryResult + Sized + Send + Sync,
    {
        let total: u64 = E::find().paginate(self.db.rdb(), 1).num_items().await?;

        let results = E::find()
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;
        Ok((results, total))
    }

    /// 获取分页数据列表及分页数
    pub async fn list_pages<E, M>(&self, page: Pagination) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        E: EntityTrait<Model = M>,
        M: FromQueryResult + Sized + Send + Sync,
    {
        let paginator = E::find().paginate(self.db.rdb(), page.page_size());
        let num_pages = paginator.num_items().await?;

        paginator
            .fetch_page(page.page())
            .await
            .map(|results| (results, num_pages))
    }
}
