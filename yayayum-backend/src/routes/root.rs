use axum::{routing::get, Router};
use sqlx::SqlitePool;

pub fn routes() -> Router<SqlitePool> {
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    tracing::info!("root handler called");
    "Hello, World!"
}