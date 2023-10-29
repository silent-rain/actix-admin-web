//!环境配置

use serde::{Deserialize, Serialize};

/// 环境配置 结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Environment {
    pub env: String, // 环境名称: prod/stag/dev
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            env: "prod".to_string(),
        }
    }
}
