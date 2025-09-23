use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    infrastructure::repositories::sqlx_repository::SqlxRepository,
    services::auth_service::AuthService,
};

pub struct AppState {
    pub repository: Arc<SqlxRepository>,
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub fn new(pg_pool: PgPool) -> Self {
        let repo = Arc::new(SqlxRepository::new(pg_pool));
        let auth_service = Arc::new(AuthService::new(repo.clone()));

        Self {
            repository: repo,
            auth_service,
        }
    }
}
