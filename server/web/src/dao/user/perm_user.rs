//! 用户管理
use crate::dto::pagination::Pagination;
use crate::dto::user::perm_user::{AddUserReq, UserListReq};

use database::DBRepo;
use entity::perm_user;
use entity::prelude::PermUser;

use nject::injectable;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, QueryFilter, Set};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};

#[injectable]
pub struct PermUserDao<'a> {
    db: &'a dyn DBRepo,
}

impl<'a> PermUserDao<'a> {
    /// 获取所有数据
    pub async fn all(&self) -> Result<Vec<perm_user::Model>, DbErr> {
        let result = PermUser::find()
            .order_by_asc(perm_user::Column::Id)
            .all(self.db.rdb())
            .await?;
        Ok(result)
    }

    /// 获取数据列表
    pub async fn list(&self, req: UserListReq) -> Result<(Vec<perm_user::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let paginator = PermUser::find()
            .order_by_asc(perm_user::Column::Id)
            .paginate(self.db.rdb(), page.page_size());

        let num_pages = paginator.num_items().await?;

        let results = paginator.fetch_page(page.page()).await?;

        Ok((results, num_pages))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_user::Model>, DbErr> {
        PermUser::find_by_id(id).one(self.db.rdb()).await
    }

    /// 添加详情信息
    pub async fn add(&self, data: AddUserReq) -> Result<perm_user::Model, DbErr> {
        let pear = perm_user::ActiveModel {
            username: Set(data.username),
            gender: Set(data.gender),
            age: Set(Some(data.age)),
            phone: Set(Some(data.phone)),
            password: Set(data.password),
            status: Set(1),
            ..Default::default() // all other attributes are `NotSet`
        };

        pear.insert(self.db.wdb()).await
    }

    /// 更新信息
    pub async fn update(&self, data: perm_user::Model) -> Result<u64, DbErr> {
        // let pear = perm_user::ActiveModel {
        //     nickname: Set(data.nickname),
        //     gender: Set(data.gender),
        //     age: Set(data.age),
        //     phone: Set(data.phone),
        //     password: Set(data.password),
        //     status: Set(data.status),
        //     ..Default::default()
        // };

        // Into ActiveModel
        let pear: perm_user::ActiveModel = data.clone().into();

        let result = PermUser::update_many()
            .set(pear)
            .filter(perm_user::Column::Id.eq(data.id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermUser::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }

    // 指定字段删除
    pub async fn delete_by_name(&self, username: String) -> Result<u64, DbErr> {
        let result = PermUser::delete_many()
            .filter(perm_user::Column::Username.contains(&username))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }
}
