use axum::{routing::post, extract::{State, Path}, http::StatusCode, Json, Router};
use sqlx::SqlitePool;
use crate::models::{CreateUser, User};

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/users", post(create_user).get(get_users))
        .route("/users/{id}", axum::routing::put(modify_user).delete(remove_user))
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

#[utoipa::path(
    put,
    path = "/users/{id}",
    request_body = CreateUser,
    params(
        ("id" = i64, Path, description = "User ID to modify")
    ),
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 404, description = "User not found")
    ),
    tag = "users"
)]
pub async fn modify_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let row = sqlx::query_as::<_, User>(
        "UPDATE users SET username = ? WHERE id = ? RETURNING id, username"
    )
    .bind(&payload.username)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(Json(row))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = i64, Path, description = "User ID to delete")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found")
    ),
    tag = "users"
)]
pub async fn remove_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}