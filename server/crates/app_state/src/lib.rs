//! 应用状态

use std::sync::Mutex;

use utils::asset::EmbedAssetTrait;

/// 内部资源共享状态
pub struct AssetState {
    pub admin_web_dist: Mutex<Box<dyn EmbedAssetTrait>>,
    pub config_file: Mutex<Box<dyn EmbedAssetTrait>>,
    pub db_data_file: Mutex<Box<dyn EmbedAssetTrait>>,
}

/// 应用共享状态
#[derive(Debug, Clone)]
pub struct AppState {}
