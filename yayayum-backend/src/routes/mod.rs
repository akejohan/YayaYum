pub mod users;
pub mod dishes;

use axum::Router;
use sqlx::SqlitePool;

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .merge(users::routes())
        .merge(dishes::routes())
}