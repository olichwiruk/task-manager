mod app;
mod domain;
mod handlers;
mod infrastructure;
mod views;

use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env or environment variable");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    let app_state = app::state::AppState::new(pool);
    let app = app::routes::create_router(app_state);

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}
