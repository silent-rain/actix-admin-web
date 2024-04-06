//! 加密解密工具集
use sha2::{Digest, Sha256};

const SECRET: &str = "secret";

/// Sha2 256 加密
pub fn sha2_256(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    hasher.update(SECRET);
    // Note that calling `finalize()` consumes hasher
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha2_256() {
        let key = sha2_256("123456xwedc");
        assert!(key == "7069cbbdd07d12dbf12dc9c858f6d11f18e4ecda89bfd8af92453b10667d3d35");
    }
}
