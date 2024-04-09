//! 资源共享状态

use rust_embed::EmbeddedFile;

#[derive(Clone)]
pub struct AssetState {
    pub asset_admin_web_dist: EmbeddedFile,
    pub asset_config_file: EmbeddedFile,
    pub asset_db_data_file: EmbeddedFile,
}
