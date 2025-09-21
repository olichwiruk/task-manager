use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::app::state::AppState;
use crate::handlers::{self, task_handler};

pub fn create_router(state: AppState) -> Router {
    let shared_state = Arc::new(state);

    Router::new()
        .route("/", get(handlers::index))
        .route("/tasks", post(task_handler::add_task))
        .with_state(shared_state)
}
