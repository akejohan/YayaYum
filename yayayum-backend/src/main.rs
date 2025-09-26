mod routes;
mod models;
mod api_doc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use api_doc::ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    println!("Listening on http://0.0.0.0:3000");

    let app = Router::new()
        .merge(routes::routes()) // import all routes
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-ui/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}