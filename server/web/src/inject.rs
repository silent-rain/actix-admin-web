//! 依赖注入

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
