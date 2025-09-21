use std::sync::Arc;

use axum::{Form, extract::State, response::IntoResponse};
use serde::Deserialize;

use crate::{
    app::state::AppState,
    domain::task::{NewTask, TaskPriority, TaskRepository},
};

#[derive(Deserialize)]
pub struct TaskForm {
    pub description: String,
    pub priority: TaskPriority,
}

pub async fn add_task(
    State(state): State<Arc<AppState>>,
    form: Form<TaskForm>,
) -> impl IntoResponse {
    let task_repo: &dyn TaskRepository = &*state.repository;
    let new_task = NewTask {
        description: form.description.clone(),
        priority: Some(form.priority.clone()),
    };
    task_repo
        .insert(new_task)
        .await
        .expect("Failed to insert task");
    "Task added"
}
