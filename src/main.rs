mod template;

use axum::{response::IntoResponse, routing::get};
use template::{HtmlTemplate, IndexTemplate};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = axum::Router::new().route("/", get(index));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}
