mod views;
mod domain;

use axum::{response::IntoResponse, routing::{get, post}, Form};
use views::templates::{HtmlTemplate, IndexTemplate};
use tokio::net::TcpListener;

use crate::domain::task::TaskForm;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = axum::Router::new()
        .route("/", get(index))
        .route("/tasks", post(add_task));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}

async fn add_task(_form: Form<TaskForm>) -> impl IntoResponse {
    "Task added"
}
