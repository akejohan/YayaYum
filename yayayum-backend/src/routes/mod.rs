pub mod root;
pub mod users;
pub mod dishes;

use axum::Router;
use sqlx::SqlitePool;

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .merge(root::routes())
        .merge(users::routes())
        .merge(dishes::routes())
}