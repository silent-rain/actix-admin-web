//! 共享状态

use database::Pool;

#[derive(Debug, Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub db: Pool,
}
