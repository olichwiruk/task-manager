use std::sync::Arc;

use axum::{
    Form,
    extract::{Path, State},
    response::IntoResponse,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    app::state::AppState,
    domain::task::{NewTask, TaskPriority, TaskRepository, TaskStatus},
    views::{HtmlTemplate, tasks::TasksTemplate},
};

#[derive(Deserialize, Validate)]
pub struct AddTaskFormData {
    #[validate(length(min = 1, max = 80))]
    pub description: String,
    pub priority: Option<TaskPriority>,
}

pub async fn add_task(
    State(state): State<Arc<AppState>>,
    form: Form<AddTaskFormData>,
) -> impl IntoResponse {
    if form.validate().is_err() {
        return get_tasks(State(state)).await;
    }

    let task_repo: &dyn TaskRepository = &*state.repository;
    let new_task =
        NewTask::new(form.description.clone(), form.priority.clone());
    task_repo
        .insert(new_task)
        .await
        .expect("Failed to insert task");

    get_tasks(State(state)).await
}

pub async fn get_tasks(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let task_repo: &dyn TaskRepository = &*state.repository;
    let tasks = task_repo.get_all().await.expect("Failed to fetch tasks");

    HtmlTemplate(TasksTemplate {
        tasks: tasks.into_iter().map(|t| t.into()).collect(),
    })
}

#[derive(Deserialize)]
pub struct UpdateTaskFormData {
    status: Option<TaskStatus>,
}

pub async fn update_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
    Form(payload): Form<UpdateTaskFormData>,
) -> impl IntoResponse {
    let task_repo: &dyn TaskRepository = &*state.repository;
    let mut task = task_repo
        .get_by_id(task_id)
        .await
        .expect("Failed to fetch task")
        .expect("Task not found");

    if let Some(status) = payload.status {
        task.change_status(status).expect("Invalid status");
    }

    task_repo.update(task).await.expect("Failed to update task");

    get_tasks(State(state)).await
}
