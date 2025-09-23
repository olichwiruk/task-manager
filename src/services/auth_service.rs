use std::sync::Arc;

use crate::domain::user::{NewUser, UserRepository};
use bcrypt::{DEFAULT_COST, hash};

pub struct AuthService {
    user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl AuthService {
    pub fn new(user_repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repo }
    }
}

impl AuthService {
    pub async fn register_user(
        &self,
        username: String,
        password: String,
    ) -> Result<(), ()> {
        let hashed_password = hash(password, DEFAULT_COST).map_err(|_| ())?;
        let new_user = NewUser {
            username,
            hashed_password,
        };

        self.user_repo.insert(new_user).await?;

        Ok(())
    }
}
