use axum::{routing::post, http::StatusCode, Json, Router};
use crate::models::{CreateUser, User};

pub fn routes() -> Router {
    Router::new().route("/users", post(create_user))
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
pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User { id: 1337, username: payload.username };
    (StatusCode::CREATED, Json(user))
}