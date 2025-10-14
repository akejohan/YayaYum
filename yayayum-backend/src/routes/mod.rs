pub mod users;
pub mod dishes;

use axum::Router;
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .merge(users::routes())
        .merge(dishes::routes())
}