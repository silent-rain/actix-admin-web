//! 访问者模式

use std::collections::BTreeMap;

use serde_json::Value;

/// 日志字段和值
#[derive(Debug, Default, Clone)]
pub struct StorageFiled {
    name: String,
    value: Value,
}

/// 自定义存储器
#[derive(Debug, Default, Clone)]
pub struct Storage {
    fileds: Vec<String>,
    metadata: BTreeMap<String, Value>,
    message: String,
}

impl Storage {
    fn add_filed(&mut self, filed: StorageFiled) {
        self.fileds.push(filed.name.clone());
        self.metadata
            .insert(filed.name.clone(), filed.value.clone());
        if filed.name == *"message" {
            self.message = serde_json::to_string(&filed.value).map_or("".to_owned(), |v| v);
        }
    }

    pub fn fileds_to_string(&self) -> Option<String> {
        serde_json::to_string(&self.fileds).ok()
    }

    pub fn metadata_to_string(&self) -> Option<String> {
        serde_json::to_string(&self.metadata).ok()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

/// 自定义访问者
#[derive(Debug, Default, Clone)]
pub struct StorageVisitor {
    storage: Storage,
}

impl StorageVisitor {
    /// 返回存储数据
    pub fn storage(&self) -> Storage {
        self.storage.clone()
    }

    /// 从存储器创建访问者
    pub fn from(storage: Storage) -> Self {
        StorageVisitor { storage }
    }
}

impl tracing::field::Visit for StorageVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.storage.add_filed(StorageFiled {
            name: field.name().to_string(),
            value: serde_json::json!(value),
        });
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.storage.add_filed(StorageFiled {
            name: field.name().to_string(),
            value: serde_json::json!(value),
        });
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.storage.add_filed(StorageFiled {
            name: field.name().to_string(),
            value: serde_json::json!(value),
        });
    }

    fn record_i128(&mut self, field: &tracing::field::Field, value: i128) {
        self.storage.add_filed(StorageFiled {
            name: field.name().to_string(),
            value: serde_json::json!(value),
        });
    }

    fn record_u128(&mut self, field: &tracing::field::Field, value: u128) {
        self.storage.add_filed(StorageFiled {
            name: field.name().to_string(),
            value: serde_json::json!(value),
        });
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.storage.add_filed(StorageFiled {
            name: field.name().to_string(),
            value: serde_json::json!(value),
        });
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.storage.add_filed(StorageFiled {
            name: field.name().to_string(),
            value: serde_json::json!(value),
        });
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        // TODO 待完善，指定存储字段
        // if let Some(v) = value.downcast_ref::<code::Error>() {
        //     self.0
        //         .insert("code".to_string(), serde_json::json!(v.code()));
        //     self.0
        //         .insert("code_msg".to_string(), serde_json::json!(v.msg()));
        //     return;
        // }

        self.storage.add_filed(StorageFiled {
            name: field.name().to_string(),
            value: serde_json::json!(value.to_string()),
        });
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.storage.add_filed(StorageFiled {
            name: field.name().to_string(),
            value: serde_json::json!(format!("{:?}", value)),
        });
    }
}
