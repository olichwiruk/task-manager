use axum::{
    Router,
    routing::{get, patch},
};
use std::sync::Arc;

use crate::app::state::AppState;
use crate::handlers::{self, task_handler};

pub fn create_router(state: AppState) -> Router {
    let shared_state = Arc::new(state);

    Router::new()
        .route("/", get(handlers::index))
        .route(
            "/tasks",
            get(task_handler::get_tasks).post(task_handler::add_task),
        )
        .route("/tasks/{id}", patch(task_handler::update_task))
        .with_state(shared_state)
}
