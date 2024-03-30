//!服务配置
use serde::{Deserialize, Serialize};

/// 服务配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    /// 服务端口配置
    pub base: Base,
    /// 上传文件配置
    pub upload: Upload,
    /// 验证码配置
    pub captcha: Captcha,
}

impl Default for Server {
    fn default() -> Server {
        Server {
            base: Base {
                address: String::from("0.0.0.0"),
                port: 8000,
            },
            upload: Upload {
                filepath: "./upload".to_string(),
            },
            captcha: Captcha { expire: 30 },
        }
    }
}

/// 服务端口配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Base {
    pub address: String,
    pub port: u32,
}
impl Base {
    /// 服务监听地址
    pub fn address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

/// 上传文件配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Upload {
    pub filepath: String,
}

/// 验证码配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Captcha {
    pub expire: i8,
}
