//!用户管理
use crate::dto::pagination::Pagination;
use crate::dto::perm_user::{AddUserReq, UserListReq};

use database::DBRepo;
use entity::perm_user;
use entity::prelude::PermUser;

use sea_orm::{ActiveModelTrait, DbErr, Set};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};

pub struct Dao<'a, DB: DBRepo> {
    db: &'a DB,
}

impl<'a, DB: DBRepo> Dao<'a, DB> {
    /// 创建对象
    pub fn new(db: &'a DB) -> Self {
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
    pub async fn list(&self, req: UserListReq) -> Result<(Vec<perm_user::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let paginator = PermUser::find()
            .order_by_asc(perm_user::Column::Id)
            .paginate(self.db.rdb(), page.page_size());

        let num_pages = paginator.num_items().await?;

        paginator
            .fetch_page(page.page())
            .await
            .map(|p| (p, num_pages))
    }

    // 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_user::Model>, DbErr> {
        PermUser::find_by_id(id).one(self.db.rdb()).await
    }

    // 添加详情信息
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

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermUser::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
