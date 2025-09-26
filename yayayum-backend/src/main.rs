use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::{ToSchema, OpenApi};

#[tokio::main]
async fn main() {
    // Log current address
    println!("Listening on http://0.0.0.0:3000");
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-ui/openapi.json", ApiDoc::openapi()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    // log to console
    tracing::info!("root handler called");
    "Hello, World!"
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created successfully", body = User)
    ),
    tag = "users"
)]
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize, ToSchema)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, ToSchema)]
struct User {
    id: u64,
    username: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
    ),
    components(
        schemas(CreateUser, User)
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
struct ApiDoc;