//! 依赖注入
use std::sync::Arc;

use database::ArcDbRepo;

use nject::provider;

#[provider]
pub struct InjectProvider {
    #[provide(ArcDbRepo, |x| x.clone())]
    adb: ArcDbRepo,
}

impl InjectProvider {
    pub fn new(db: ArcDbRepo) -> Self {
        InjectProvider { adb: db }
    }

    pub fn anew(db: ArcDbRepo) -> Arc<Self> {
        let provide = InjectProvider { adb: db };

        Arc::new(provide)
    }

    // pub fn new() -> Self {
    //     #[provider]
    //     struct InitInjectProvider;

    //     InitInjectProvider.provide::<InjectProvider>()
    // }
}

pub type AInjectProvider = Arc<InjectProvider>;
