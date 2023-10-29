//! 共享状态

use database::Pool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Pool,
}
