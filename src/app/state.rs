use sqlx::PgPool;
use std::sync::Arc;

use crate::infrastructure::repositories::sqlx_repository::SqlxRepository;

pub struct AppState {
    pub repository: Arc<SqlxRepository>,
}

impl AppState {
    pub fn new(pg_pool: PgPool) -> Self {
        let repo = SqlxRepository::new(pg_pool);
        Self {
            repository: Arc::new(repo),
        }
    }
}
