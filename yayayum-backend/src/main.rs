mod api_doc;
mod models;
mod routes;

use api_doc::ApiDoc;
use axum::{Router, http::Method, response::Redirect, routing::get};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool
) -> shuttle_axum::ShuttleAxum {
    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let static_files = Router::new()
        .route("/manage", get(|| async { Redirect::to("/?view=manage") }))
        .merge(Router::new().fallback_service(ServeDir::new("assets")));

    // Build router
    let router = Router::new()
        .merge(routes::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-ui/openapi.json", ApiDoc::openapi()))
        .merge(static_files)
        .layer(cors)
        .with_state(pool);

    Ok(router.into())
}
