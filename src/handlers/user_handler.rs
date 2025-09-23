use axum::{
    Form, Json, extract::State, http::StatusCode, response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::app::state::AppState;

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

#[derive(Deserialize)]
pub struct LoginUserData {
    pub username: String,
    pub password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    user_data: Form<LoginUserData>,
) -> impl IntoResponse {
    let auth_service = &state.auth_service;

    let authentication_result = auth_service
        .authenticate_user(
            user_data.username.clone(),
            user_data.password.clone(),
        )
        .await;

    if authentication_result.is_ok() {
        (StatusCode::OK, [("HX-Redirect", "/")], "").into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({"status": "error", "message": "Invalid credentials"})),
        )
            .into_response()
    }
}
