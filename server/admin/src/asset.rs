//! 静态资源文件
#![allow(unused)]

use utils::asset::EmbedAssetTrait;

use rust_embed::{EmbeddedFile, RustEmbed};

/// sqlte3 数据库
/// filepath: data.dat
#[derive(Debug, Clone, Default, RustEmbed)]
#[folder = "."]
#[include = "data.dat"]
pub struct AssetDbDataFile;

impl EmbedAssetTrait for AssetDbDataFile {
    fn get(&self, file_path: &str) -> Option<EmbeddedFile> {
        Self::get(file_path)
    }
}

/// 配置文件
/// filepath: config.yaml
#[derive(Debug, Clone, Default, RustEmbed)]
#[folder = "."]
#[include = "config.yaml"]
pub struct AssetConfigFile;

impl EmbedAssetTrait for AssetConfigFile {
    fn get(&self, file_path: &str) -> Option<EmbeddedFile> {
        Self::get(file_path)
    }
}

/// 后台管理 WEB 静态资源
#[derive(Debug, Clone, Default, RustEmbed)]
#[folder = "../../web/dist/"]
pub struct AssetAdminWebDist;

impl EmbedAssetTrait for AssetAdminWebDist {
    fn get(&self, file_path: &str) -> Option<EmbeddedFile> {
        Self::get(file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let asset = AssetConfigFile.data("config.yaml");
        assert!(asset.is_some());
    }
}
