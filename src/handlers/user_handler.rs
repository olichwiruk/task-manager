use axum::{
    Form, Json,
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
};
use cookie::{Cookie, SameSite};
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

    if let Ok(jwt) = authentication_result {
        let cookie = Cookie::build(("jwt", jwt))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .path("/")
            .build();

        (
            StatusCode::OK,
            [
                (header::SET_COOKIE.to_string(), cookie.to_string()),
                ("HX-Redirect".to_string(), "/".to_owned()),
            ],
            "",
        )
            .into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({"status": "error", "message": "Invalid credentials"})),
        )
            .into_response()
    }
}

pub async fn logout() -> impl IntoResponse {
    let cookie = Cookie::build(("jwt", ""))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(cookie::time::Duration::ZERO)
        .build();

    (
        StatusCode::SEE_OTHER,
        [
            (header::SET_COOKIE, cookie.to_string()),
            (header::LOCATION, "/".to_string()),
        ],
        "",
    )
        .into_response()
}
