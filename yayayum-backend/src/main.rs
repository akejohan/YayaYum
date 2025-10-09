mod api_doc;
mod models;
mod routes;

use api_doc::ApiDoc;
use axum::{Router, http::Method, response::Redirect, routing::get};
use shuttle_runtime::SecretStore;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let db_url = secrets
        .get("DATABASE_URL")
        .unwrap_or_else(|| "sqlite://yayayum.db".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

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

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let static_files = Router::new()
        .route("/manage", get(|| async { Redirect::to("/?view=manage") })) // Manage route
        .merge(Router::new().fallback_service(ServeDir::new("assets"))); // Static files

    // Build router
    let router = Router::new()
        .merge(routes::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-ui/openapi.json", ApiDoc::openapi()))
        .merge(static_files)
        .layer(cors)
        .with_state(pool);

    Ok(router.into())
}
