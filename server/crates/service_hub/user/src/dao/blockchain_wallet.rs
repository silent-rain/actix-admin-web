//! 用户区块链钱包管理
use crate::dto::blockchain_wallet::GetBlockchainWalletListReq;

use database::{DbRepo, Pagination};
use entity::user::{blockchain_wallet, BlockchainWallet};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct BlockchainWalletDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> BlockchainWalletDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetBlockchainWalletListReq,
    ) -> Result<(Vec<blockchain_wallet::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = BlockchainWallet::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(blockchain_wallet::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(blockchain_wallet::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(blockchain_wallet::Column::UserId.eq(v))
            })
            .apply_if(req.wallet_address, |query, v| {
                query.filter(blockchain_wallet::Column::WalletAddress.eq(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(blockchain_wallet::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<blockchain_wallet::Model>, DbErr> {
        BlockchainWallet::find_by_id(id).one(self.db.rdb()).await
    }

    /// 通过钱包地址获取详情信息
    pub async fn info_by_wallet_address(
        &self,
        wallet_address: String,
    ) -> Result<Option<blockchain_wallet::Model>, DbErr> {
        BlockchainWallet::find()
            .filter(blockchain_wallet::Column::WalletAddress.eq(wallet_address))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: blockchain_wallet::ActiveModel,
    ) -> Result<blockchain_wallet::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: blockchain_wallet::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = BlockchainWallet::update_many()
            .set(active_model)
            .filter(blockchain_wallet::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = BlockchainWallet::delete_by_id(id)
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}
