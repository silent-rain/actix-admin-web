//! 静态资源定义
use std::io::Write;

use code::Error;

use rust_embed::EmbeddedFile;

/// 内嵌资源
pub trait EmbedAssetTrait: Send + 'static {
    /// 获取内置文件对象
    fn get(&self, file_path: &str) -> Option<EmbeddedFile>;

    /// 获取文件二进制数据
    fn data(&self, file_path: &str) -> Option<Vec<u8>> {
        self.get(file_path).map(|asset| asset.data.to_vec())
    }

    /// 获取文件类型
    fn mimetype(&self, file_path: &str) -> Option<String> {
        self.get(file_path)
            .map(|asset| asset.metadata.mimetype().to_string())
    }

    /// 写入文件, 如存在则覆盖文件
    fn write(&self, file_path: &str) -> Result<(), Error> {
        let data = self
            .get(file_path)
            .map(|asset| asset.data.to_vec())
            .ok_or(Error::AssetNotFound)?;
        let mut file = std::fs::File::create(file_path)?;
        file.write_all(&data)?;
        Ok(())
    }

    /// 转为字符串
    fn to_string(&self, file_path: &str) -> Result<String, Error> {
        let data = self
            .get(file_path)
            .map(|asset| asset.data.to_vec())
            .ok_or(Error::AssetNotFound)?;
        let content = String::from_utf8(data)?;
        Ok(content)
    }
}
