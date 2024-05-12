//! 用户区块链钱包管理
use crate::{
    dao::blockchain_wallet::BlockchainWalletDao,
    dto::blockchain_wallet::{
        AddBlockchainWalletReq, GetBlockchainWalletListReq, UpdateBlockchainWalletReq,
    },
};

use code::{Error, ErrorMsg};
use entity::user::blockchain_wallet;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct BlockchainWalletService<'a> {
    blockchain_wallet_dao: BlockchainWalletDao<'a>,
}

impl<'a> BlockchainWalletService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetBlockchainWalletListReq,
    ) -> Result<(Vec<blockchain_wallet::Model>, u64), ErrorMsg> {
        let (mut results, total) = self.blockchain_wallet_dao.list(req).await.map_err(|err| {
            error!("查询用户区块链钱包列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询用户区块链钱包列表失败")
        })?;

        // 屏蔽敏感信息
        for result in results.iter_mut() {
            result.mnemonic = None;
            result.private_key = None;
        }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<blockchain_wallet::Model, ErrorMsg> {
        let mut result = self
            .blockchain_wallet_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询用户区块链钱包信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询用户区块链钱包信息失败")
            })?
            .ok_or_else(|| {
                error!("用户区块链钱包不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("用户区块链钱包不存在")
            })?;

        // 屏蔽敏感信息
        result.mnemonic = None;
        result.private_key = None;
        Ok(result)
    }

    /// 添加数据
    pub async fn add(
        &self,
        req: AddBlockchainWalletReq,
    ) -> Result<blockchain_wallet::Model, ErrorMsg> {
        // 查询用户区块链钱包是否已存在
        self.check_username(req.wallet_address.clone()).await?;

        let model = blockchain_wallet::ActiveModel {
            user_id: Set(req.user_id),
            wallet_address: Set(req.wallet_address),
            mnemonic: Set(req.mnemonic),
            private_key: Set(req.private_key),
            chain_id: Set(req.chain_id),
            desc: Set(req.desc),
            ..Default::default()
        };
        let result = self.blockchain_wallet_dao.add(model).await.map_err(|err| {
            error!("添加用户区块链钱包信息失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加用户区块链钱包信息失败")
        })?;

        Ok(result)
    }

    /// 更新用户区块链钱包
    pub async fn update(&self, id: i32, req: UpdateBlockchainWalletReq) -> Result<u64, ErrorMsg> {
        let model = blockchain_wallet::ActiveModel {
            id: Set(id),
            desc: Set(req.desc),
            ..Default::default()
        };

        let result = self
            .blockchain_wallet_dao
            .update(model)
            .await
            .map_err(|err| {
                error!("更新用户区块链钱包失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新用户区块链钱包失败")
            })?;

        Ok(result)
    }

    /// 检查用户区块链钱包, 查看用户区块链钱包是否已注册
    async fn check_username(&self, wallet_address: String) -> Result<(), ErrorMsg> {
        // 查询用户区块链钱包是否已存在
        let blockchain_wallet = self
            .blockchain_wallet_dao
            .info_by_wallet_address(wallet_address)
            .await
            .map_err(|err| {
                error!("查询用户区块链钱包信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询用户区块链钱包信息失败")
            })?;
        if blockchain_wallet.is_some() {
            error!("用户区块链钱包已存在");
            return Err(Error::DbDataExistError
                .into_msg()
                .with_msg("用户区块链钱包已存在"));
        }
        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.blockchain_wallet_dao.delete(id).await.map_err(|err| {
            error!("删除用户区块链钱包信息失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除用户区块链钱包信息失败")
        })?;

        Ok(result)
    }
}
