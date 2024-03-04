//! 依赖注入
use std::sync::Arc;

use database::{DBRepo, Pool};

use nject::provider;

#[provider]
pub struct Provider {
    #[provide(dyn DBRepo)]
    db: Pool,
}

impl Provider {
    pub fn new(db: Pool) -> Self {
        Provider { db }
    }

    // pub fn new() -> Self {
    //     #[provider]
    //     struct InitProvider;

    //     InitProvider.provide::<Provider>()
    // }
}

pub type AProvider = Arc<Provider>;
