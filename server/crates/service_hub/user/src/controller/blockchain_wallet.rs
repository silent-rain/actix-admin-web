//! 用户区块链钱包管理

use crate::{
    dto::blockchain_wallet::{
        AddBlockchainWalletReq, GetBlockchainWalletListReq, UpdateBlockchainWalletReq,
    },
    service::blockchain_wallet::BlockchainWalletService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct BlockchainWalletController;

impl BlockchainWalletController {
    /// 获取用户区块链钱包列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetBlockchainWalletListReq>,
    ) -> impl Responder {
        let blockchain_wallet_service: BlockchainWalletService = provider.provide();
        let resp = blockchain_wallet_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取用户区块链钱包信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let blockchain_wallet_service: BlockchainWalletService = provider.provide();
        let resp = blockchain_wallet_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加用户区块链钱包
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddBlockchainWalletReq>,
    ) -> impl Responder {
        let blockchain_wallet_service: BlockchainWalletService = provider.provide();
        let resp = blockchain_wallet_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户区块链钱包
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateBlockchainWalletReq>,
    ) -> impl Responder {
        let blockchain_wallet_service: BlockchainWalletService = provider.provide();
        let resp = blockchain_wallet_service
            .update(*id, data.into_inner())
            .await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除用户区块链钱包
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let blockchain_wallet_service: BlockchainWalletService = provider.provide();
        let resp = blockchain_wallet_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
