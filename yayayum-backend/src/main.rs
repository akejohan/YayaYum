mod api_doc;
mod models;
mod routes;

use api_doc::ApiDoc;
use axum::http::Method;
use axum::{Router, http};
use tower_http::cors::Any;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    println!("Listening on http://0.0.0.0:3000");

    // Add CORS layer
    let layer = tower_http::cors::CorsLayer::new()
        .allow_origin(http::HeaderValue::from_static("http://localhost:5173"))
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .merge(routes::routes()) // import all routes
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-ui/openapi.json", ApiDoc::openapi()))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
