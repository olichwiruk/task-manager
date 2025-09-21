pub mod task_handler;

use axum::response::IntoResponse;
use crate::views::templates::{HtmlTemplate, IndexTemplate};

pub async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}
