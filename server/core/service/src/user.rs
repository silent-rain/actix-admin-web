//! 用户管理
use code::Error;
use dao::user::Dao;
use database::Pool;
use dto::perm_user::AddUserReq;
use entity::perm_user::Model;

/// 用户服务
pub struct Service;

impl Service {
    /// 获取列表数据
    pub async fn list(db: &Pool, page: u64, page_size: u64) -> Result<(Vec<Model>, u64), Error> {
        let results = Dao::new(db).list(page, page_size).await.map_err(|e| {
            println!("========== {:#?}", e);
            Error::InternalServerError
        })?;
        Ok(results)
    }

    /// 获取详情数据
    pub async fn info(db: &Pool, id: i32) -> Result<Option<Model>, Error> {
        let result = Dao::new(db).info(id).await.map_err(|e| {
            println!("========== {:#?}", e);
            Error::InternalServerError
        })?;
        Ok(result)
    }

    /// 添加数据
    pub async fn add(db: &Pool, data: AddUserReq) -> Result<Model, Error> {
        let result = Dao::new(db).add(data).await.map_err(|e| {
            println!("========== {:#?}", e);
            Error::InternalServerError
        })?;
        Ok(result)
    }
}
