use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    infrastructure::repositories::sqlx_repository::SqlxRepository,
    services::auth_service::AuthService,
};

pub struct AppState {
    pub jwt_secret: String,
    pub repository: Arc<SqlxRepository>,
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub fn new(pg_pool: PgPool, jwt_secret: String) -> Self {
        let repo = Arc::new(SqlxRepository::new(pg_pool));
        let auth_service =
            Arc::new(AuthService::new(jwt_secret.clone(), repo.clone()));

        Self {
            jwt_secret,
            repository: repo,
            auth_service,
        }
    }
}
