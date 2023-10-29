//!用户管理
use database::Pool;
use dto::perm_user::AddUserReq;
use entity::perm_user;
use entity::prelude::PermUser;

use sea_orm::{ActiveModelTrait, DbErr, Set};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};

pub struct Dao<'a> {
    db: &'a Pool,
}

impl<'a> Dao<'a> {
    /// 创建对象
    pub fn new(db: &'a Pool) -> Self {
        Dao { db }
    }

    /// 获取所有数据
    pub async fn all(&self) -> Result<Vec<perm_user::Model>, DbErr> {
        let result = PermUser::find()
            .order_by_asc(perm_user::Column::Id)
            .all(self.db.rdb())
            .await?;
        Ok(result)
    }

    // 获取数据列表
    pub async fn list(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<perm_user::Model>, u64), DbErr> {
        let paginator = PermUser::find()
            .order_by_asc(perm_user::Column::Id)
            .paginate(self.db.rdb(), page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page).await.map(|p| (p, num_pages))
    }

    // 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_user::Model>, DbErr> {
        PermUser::find_by_id(id).one(self.db.rdb()).await
    }

    // 插入一个活动模型并返回一个新的 Model .其值是从数据库中检索的，因此将填充任何自动生成的字段。
    pub async fn add(&self, data: AddUserReq) -> Result<perm_user::Model, DbErr> {
        let pear = perm_user::ActiveModel {
            nickname: Set(data.nickname),
            gender: Set(data.gender),
            age: Set(Some(data.age)),
            phone: Set(Some(data.phone)),
            password: Set(data.password),
            status: Set(1),
            ..Default::default() // all other attributes are `NotSet`
        };

        pear.insert(self.db.rdb()).await
    }
}
