//! 静态资源文件
#![allow(unused)]

use rust_embed::RustEmbed;

/// sqlte3 数据库
#[derive(Debug, Default, RustEmbed)]
#[folder = "../../"]
#[include = "data.dat"]
pub struct AssetDbDataFile;

impl AssetDbDataFile {
    #[allow(unused)]
    pub fn to_bytes() -> Option<Vec<u8>> {
        let asset = Self::get("data.dat")?;
        Some(asset.data.to_vec())
    }
}

/// 配置文件
#[derive(Debug, Default, RustEmbed)]
#[folder = "../"]
#[include = "config.toml"]
pub struct AssetConfigFile;

impl AssetConfigFile {
    /// 转换为字节矢量
    #[allow(unused)]
    pub fn to_bytes() -> Option<Vec<u8>> {
        let asset = Self::get("config.toml")?;
        Some(asset.data.to_vec())
    }
}

/// WEB 静态资源
#[derive(Debug, Default, RustEmbed)]
#[folder = "../../web/dist/"]
pub struct AssetWebDist;

impl AssetWebDist {
    pub fn to_bytes(path: String) -> Option<Vec<u8>> {
        let asset = Self::get(&path)?;
        Some(asset.data.to_vec())
    }

    /// 获取文件类型
    pub fn mimetype(path: String) -> Option<String> {
        let asset = Self::get(&path)?;
        Some(asset.metadata.mimetype().to_string())
    }
}
