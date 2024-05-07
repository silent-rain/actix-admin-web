//! 应用状态

use utils::asset::EmbedAssetTrait;

use tokio::sync::RwLock;

/// 内部资源共享状态
pub struct AssetState {
    pub admin_web_dist: RwLock<Box<dyn EmbedAssetTrait + Send + Sync + 'static>>,
    pub config_file: RwLock<Box<dyn EmbedAssetTrait + Send + Sync + 'static>>,
    pub db_data_file: RwLock<Box<dyn EmbedAssetTrait + Send + Sync + 'static>>,
}

/// 应用共享状态
#[derive(Debug, Clone)]
pub struct AppState {}
