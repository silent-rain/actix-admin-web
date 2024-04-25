//! 依赖注入
use std::sync::Arc;

use database::{DbRepo, Pool};

use nject::provider;

#[provider]
pub struct InjectProvider {
    #[provide(dyn DbRepo)]
    db: Pool,
}

impl InjectProvider {
    pub fn new(db: Pool) -> Self {
        InjectProvider { db }
    }

    pub fn anew(db: Pool) -> Arc<Self> {
        let provide = InjectProvider { db };

        Arc::new(provide)
    }

    // pub fn new() -> Self {
    //     #[provider]
    //     struct InitInjectProvider;

    //     InitInjectProvider.provide::<InjectProvider>()
    // }
}

pub type AInjectProvider = Arc<InjectProvider>;
