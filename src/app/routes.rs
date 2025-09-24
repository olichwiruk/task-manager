use axum::{
    Router, middleware,
    routing::{get, patch, post},
};
use std::sync::Arc;

use crate::{
    app::state::AppState,
    handlers::{
        self, auth::middleware::auth_middleware, task_handler, user_handler,
    },
};

pub fn create_router(state: AppState) -> Router {
    let shared_state = Arc::new(state);

    let protected_routes = Router::new()
        .route("/", get(handlers::index))
        .route("/logout", get(user_handler::logout))
        .route(
            "/tasks",
            get(task_handler::get_tasks).post(task_handler::add_task),
        )
        .route("/tasks/{id}", patch(task_handler::update_task))
        .layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/register", post(user_handler::register))
        .route(
            "/login",
            get(handlers::login_page).post(user_handler::login),
        )
        .merge(protected_routes)
        .with_state(shared_state)
}
