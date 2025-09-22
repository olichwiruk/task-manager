use axum::{Json, extract::State, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::{
    app::state::AppState,
    domain::user::{NewUser, UserRepository},
};

#[derive(Deserialize)]
pub struct RegisterUserData {
    pub username: String,
    pub password: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    user_data: Json<RegisterUserData>,
) -> impl IntoResponse {
    let user_repo: &dyn UserRepository = &*state.repository;

    let hashed_password = hash(user_data.password.clone(), DEFAULT_COST)
        .expect("Failed to hash password");
    let new_user = NewUser {
        username: user_data.username.clone(),
        hashed_password,
    };
    user_repo
        .insert(new_user)
        .await
        .expect("Failed to insert task");

    Json(json!({"status": "success"}))
}
