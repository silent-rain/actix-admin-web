//! 静态资源文件
#![allow(unused)]

use utils::asset::EmbedAssetTrait;

use rust_embed::{EmbeddedFile, RustEmbed};

/// 库表资源
#[derive(Debug, Clone, Default, RustEmbed)]
#[folder = "./resources/table"]
pub struct AssetDbTable;

impl EmbedAssetTrait for AssetDbTable {
    fn get(&self, file_path: &str) -> Option<EmbeddedFile> {
        Self::get(file_path)
    }
}

/// 表数据资源
#[derive(Debug, Clone, Default, RustEmbed)]
#[folder = "./resources/table_data"]
pub struct AssetDbTableData;

impl EmbedAssetTrait for AssetDbTableData {
    fn get(&self, file_path: &str) -> Option<EmbeddedFile> {
        Self::get(file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let t_user_role = "t_user_role.sql";
        let asset = AssetDbTable;
        let data = asset.get(t_user_role);
        assert!(data.is_some());
        let content = asset.to_string(t_user_role);
        println!("content: {:#?}", content);
        assert!(content.is_ok());
    }
}
