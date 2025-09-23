use axum::{Json, extract::State, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::{
    app::state::AppState,
    domain::user::UserRepository,
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
    let auth_service = &state.auth_service;

    auth_service
        .register_user(user_data.username.clone(), user_data.password.clone())
        .await
        .expect("Failed to register user");

    Json(json!({"status": "success"}))
}
