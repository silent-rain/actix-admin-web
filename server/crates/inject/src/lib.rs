//! 依赖注入
use std::sync::Arc;

use database::PoolTrait;

use nject::provider;

#[provider]
pub struct InjectProvider {
    #[provide(Arc<dyn PoolTrait>, |x| x.clone())]
    adb: Arc<dyn PoolTrait>,
}

impl InjectProvider {
    pub fn new(db: Arc<dyn PoolTrait>) -> Self {
        InjectProvider { adb: db }
    }
}

pub type AInjectProvider = Arc<InjectProvider>;

// 实现自定义 Injectable trait
// impl<'a> Injectable<'a, Arc<dyn PoolTrait>, InjectProvider> for Arc<dyn PoolTrait> {
//     fn inject(provider: &'a InjectProvider) -> Self {
//         provider.db.clone()
//     }
// }
