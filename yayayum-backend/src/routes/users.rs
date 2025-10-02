use axum::{routing::post, extract::State, http::StatusCode, Json, Router};
use sqlx::SqlitePool;
use crate::models::{CreateUser, User};

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/users", post(create_user).get(get_users))
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses((status = 201, description = "User created", body = User)),
    tag = "users"
)]
pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let row = sqlx::query_as::<_, User>(
        "INSERT INTO users (username) VALUES (?) RETURNING id, username"
    )
    .bind(&payload.username)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(row)))
}

#[utoipa::path(
    get,
    path = "/users",
    responses((status = 200, description = "List users", body = [User])),
    tag = "users"
)]
pub async fn get_users(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let rows = sqlx::query_as::<_, User>("SELECT id, username FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rows))
}