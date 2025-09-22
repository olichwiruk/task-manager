pub mod task_handler;
pub mod user_handler;

use axum::response::IntoResponse;
use crate::views::{HtmlTemplate, IndexTemplate};

pub async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}
