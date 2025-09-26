use axum::{routing::post, http::StatusCode, Json, Router};
use crate::models::{CreateUser, User};
use std::fs::{OpenOptions};
use std::io::{Write, BufRead, BufReader};

pub fn routes() -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_users))
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

    // Append user to a file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("users.txt")
        .expect("Cannot open users.txt");

    let line = format!("{};{}\n", user.id, user.username);
    file.write_all(line.as_bytes()).expect("Failed to write to file");

    (StatusCode::CREATED, Json(user))
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List all users", body = [User])
    ),
    tag = "users"
)]
pub async fn get_users() -> Json<Vec<User>> {
    let mut users = Vec::new();

    if let Ok(file) = OpenOptions::new().read(true).open("users.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if let Some((id, username)) = line.split_once(';') {
                if let Ok(id) = id.parse::<u64>() {
                    users.push(User { id, username: username.to_string() });
                }
            }
        }
    }

    Json(users)
}
