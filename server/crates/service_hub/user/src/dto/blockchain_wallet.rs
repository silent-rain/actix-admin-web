//! 用户区块链钱包管理

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询用户区块链钱包列表
#[derive(Default, Deserialize, Validate)]
pub struct GetBlockchainWalletListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户ID
    pub user_id: Option<i32>,
    /// 钱包地址
    pub wallet_address: Option<String>,
}

/// 添加用户区块链钱包
#[derive(Serialize, Deserialize, Validate)]
pub struct AddBlockchainWalletReq {
    /// 用户ID
    pub user_id: i32,
    /// 钱包地址
    pub wallet_address: String,
    /// 助记词
    pub mnemonic: Option<String>,
    /// 私钥
    pub private_key: Option<String>,
    /// 区块链ID
    pub chain_id: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateBlockchainWalletReq {
    /// 区块链ID
    pub chain_id: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
}
