mod domain;
mod views;

use axum::{
    Form,
    response::IntoResponse,
    routing::{get, post},
};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::{env, sync::Arc};
use tokio::net::TcpListener;
use views::templates::{HtmlTemplate, IndexTemplate};

use crate::domain::task::TaskForm;

struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env or environment variable");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    let shared_state = Arc::new(AppState { db: pool });

    let app = axum::Router::new()
        .route("/", get(index))
        .route("/tasks", post(add_task))
        .with_state(shared_state);

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
