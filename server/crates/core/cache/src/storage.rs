use std::{
    sync::OnceLock,
    time::{Duration, Instant},
};

use moka::future::Cache as MokaCache;
use serde::Serialize;
use serde_json::{json, Value};

/// 全局调度对象，用于存储和管理缓存实例。
static GLOBAL_SCHED: OnceLock<MokaCache<String, Entry>> = OnceLock::new();

/// 缓存条目结构体，包含一个值和一个可选的过期时间戳。
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub value: Value, // 存储的值，使用serde_json::Value以支持多种数据类型。
    pub expires_at: Option<Instant>, // 可选的过期时间戳。
}

impl Entry {
    /// 创建一个新的 Entry 实例，不带过期时间。
    fn new<T: Serialize>(value: T) -> Self {
        Entry {
            value: json!(value), // 将值序列化为JSON。
            expires_at: None,    // 没有过期时间。
        }
    }

    /// 创建一个新的 Entry 实例，带有指定的过期时间。
    fn new_with_ttl<T: Serialize>(value: T, ttl: Duration) -> Self {
        Entry {
            value: json!(value),                    // 将值序列化为JSON。
            expires_at: Some(Instant::now() + ttl), // 设置过期时间。
        }
    }

    /// 检查条目是否已过期。
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expiry) => Instant::now() >= expiry, // 如果当前时间超过过期时间，则过期。
            None => false,                            // 永不过期。
        }
    }
}

/// 缓存配置结构体
pub struct CacheConfig {
    ttl_ms: u64,
    max_capacity: u64,
}

// 缓存构建器
pub struct CacheBuilder {
    config: CacheConfig,
}

impl CacheBuilder {
    // 创建一个新的构建器实例
    pub fn new() -> Self {
        CacheBuilder {
            config: CacheConfig {
                ttl_ms: 10000,
                max_capacity: 10000,
            },
        }
    }

    // 设置过期时间
    pub fn with_ttl_ms(mut self, ttl_ms: u64) -> Self {
        self.config.ttl_ms = ttl_ms;
        self
    }

    // 设置最大容量
    pub fn with_max_capacity(mut self, max_capacity: u64) -> Self {
        self.config.max_capacity = max_capacity;
        self
    }

    // 构建缓存实例
    pub fn build(self) -> Cache {
        let cache = GLOBAL_SCHED.get_or_init(|| {
            MokaCache::builder()
                .max_capacity(self.config.max_capacity)
                .time_to_live(Duration::from_millis(self.config.ttl_ms))
                .build()
        });
        Cache {
            cache: cache.clone(),
        }
    }
}

/// 缓存结构体，包含Moka缓存实例和配置参数。
pub struct Cache {
    cache: MokaCache<String, Entry>, // Moka缓存实例。
}

impl Default for Cache {
    /// 默认构造函数，初始化缓存或获取现有的缓存对象。
    fn default() -> Self {
        let cache = GLOBAL_SCHED.get_or_init(|| {
            MokaCache::builder()
                .max_capacity(10000) // 设置最大容量。
                .time_to_live(Duration::from_millis(10000)) // 设置默认的全局过期时间。
                .build() // 构建缓存。
        });
        Cache {
            cache: cache.clone(), // 克隆缓存实例。
        }
    }
}

impl Cache {
    /// 获取全局对象
    pub fn global() -> Option<Self> {
        GLOBAL_SCHED.get().map(|v| Cache { cache: v.clone() })
    }

    /// 设置缓存条目。
    pub async fn set<T: Serialize>(&self, key: &str, value: T) {
        self.cache.insert(key.to_string(), Entry::new(value)).await; // 插入新的缓存条目。
    }

    /// 获取缓存条目。
    pub async fn get(&self, key: &str) -> Option<Value> {
        self.cache
            .get(&key.to_string())
            .await
            .map(|entry| entry.value) // 获取缓存条目的值。
    }

    /// 设置缓存条目并指定过期时间。
    pub async fn set_with_expiry<T: Serialize>(&self, key: &str, value: T, ttl: Duration) {
        let timed_entry = Entry::new_with_ttl(value, ttl); // 创建带有过期时间的缓存条目。
        self.cache.insert(key.to_string(), timed_entry).await; // 插入缓存条目。
    }

    /// 获取缓存条目，如果过期则移除。
    pub async fn get_with_expiry(&self, key: &str) -> Option<Entry> {
        if let Some(entry) = self.cache.get(&key.to_string()).await {
            if entry.is_expired() {
                self.cache.invalidate(&key.to_string()).await; // 如果过期，则移除缓存条目。
                None
            } else {
                Some(entry) // 如果未过期，则返回缓存条目。
            }
        } else {
            None // 如果缓存条目不存在，则返回None。
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time;

    use super::*;

    #[tokio::test]
    async fn test_cache() {
        let cache = Cache::default();

        cache.set("silent", "rain").await;

        assert!(cache.get("silent").await == Some(json!("rain")));
    }

    #[tokio::test]
    async fn test_cache_expiry() {
        let cache = Cache::default();

        cache
            .set_with_expiry("silent", "rain".to_string(), time::Duration::from_secs(5))
            .await;

        tokio::time::sleep(time::Duration::from_secs(2)).await;

        assert!(cache.get("silent").await == Some(json!("rain")));

        let result = cache.get_with_expiry("silent").await.expect("获取缓存失败");
        assert!(!result.is_expired());
        // println!("result: {:#?}", result);

        tokio::time::sleep(time::Duration::from_secs(3)).await;

        assert!((cache.get_with_expiry("silent").await).is_none());
    }
}
