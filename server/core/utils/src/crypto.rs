//! 加密解密工具集
use crypto::{digest::Digest, md5::Md5};

const SECRET: &str = "secret";

/// Md5 加密
pub fn make_md5(text: &str) -> String {
    let mut hasher = Md5::new();
    let text = String::from(text);
    // 将密码和盐连接起来
    let salted_text = format!("{}{}", text, SECRET);
    hasher.input_str(&salted_text);
    
    hasher.result_str()
}
