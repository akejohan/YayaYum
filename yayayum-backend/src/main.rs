mod api_doc;
mod models;
mod routes;

use api_doc::ApiDoc;
use axum::http::Method;
use axum::{Router, http};
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::Any;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();
    println!("Listening on http://0.0.0.0:3000");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://yayayum.db")
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Could not create users table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS dishes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            price_kr INTEGER NOT NULL,
            dietary_restrictions TEXT NOT NULL,
            category TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Could not create dishes table");

    // Add CORS layer
    let layer = tower_http::cors::CorsLayer::new()
        .allow_origin(http::HeaderValue::from_static("http://localhost:5173"))
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .merge(routes::routes()) // import all routes
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-ui/openapi.json", ApiDoc::openapi()))
        .layer(layer)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
