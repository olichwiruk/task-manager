pub mod auth;
pub mod task_handler;
pub mod user_handler;

use crate::views::{HtmlTemplate, IndexTemplate, LoginTemplate};
use axum::response::IntoResponse;

pub async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}

pub async fn login_page() -> impl IntoResponse {
    HtmlTemplate(LoginTemplate {})
}
