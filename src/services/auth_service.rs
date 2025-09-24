use std::sync::Arc;

use crate::domain::user::{NewUser, UserRepository};
use bcrypt::{DEFAULT_COST, hash};
use jsonwebtoken::EncodingKey;

pub struct AuthService {
    jwt_secret: String,
    user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl AuthService {
    pub fn new(
        jwt_secret: String,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    ) -> Self {
        Self {
            jwt_secret,
            user_repo,
        }
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

    pub async fn authenticate_user(
        &self,
        username: String,
        password: String,
    ) -> Result<String, ()> {
        let user = self.user_repo.get_by_username(&username).await?;

        if let Some(user) = user
            && bcrypt::verify(password, &user.hashed_password)
                .map_err(|_| ())?
        {
            let claims = Claims {
                sub: user.username,
                exp: (chrono::Utc::now() + chrono::Duration::hours(24))
                    .timestamp() as usize,
            };
            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
            )
            .map_err(|_| ())?;

            return Ok(token);
        }

        Err(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
