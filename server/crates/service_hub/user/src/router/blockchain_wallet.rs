//! 用户区块链钱包管理

use crate::controller::blockchain_wallet::BlockchainWalletController;

use actix_web::{web, Scope};

/// 路由器
pub struct BlockchainWalletRouter;

impl BlockchainWalletRouter {
    /// 注册`用户区块链钱包管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/blockchain-wallets")
            .route("", web::get().to(BlockchainWalletController::list))
            .route("/{id}", web::get().to(BlockchainWalletController::info))
            .route("", web::post().to(BlockchainWalletController::add))
            .route("/{id}", web::put().to(BlockchainWalletController::update))
            .route(
                "/{id}",
                web::delete().to(BlockchainWalletController::delete),
            )
    }
}
