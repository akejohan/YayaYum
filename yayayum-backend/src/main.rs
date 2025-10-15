mod api_doc;
mod models;
mod routes;

use api_doc::ApiDoc;
use axum::{Router, http::Method};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool
) -> shuttle_axum::ShuttleAxum {
    // Create tables manually
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS dishes (
            id SERIAL PRIMARY KEY,
            nr INTEGER NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            price_kr INTEGER NOT NULL,
            dietary_restrictions TEXT NOT NULL,
            category TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create dishes table");

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_dishes_category ON dishes(category)")
        .execute(&pool)
        .await
        .ok();
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_dishes_nr ON dishes(nr)")
        .execute(&pool)
        .await
        .ok();

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let static_files = Router::new()
        .fallback_service(ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html")));

    // Build router
    let router = Router::new()
        .merge(routes::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-ui/openapi.json", ApiDoc::openapi()))
        .merge(static_files)
        .layer(cors)
        .with_state(pool);

    Ok(router.into())
}
