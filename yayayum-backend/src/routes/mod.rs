pub mod root;
pub mod users;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(root::routes())
        .merge(users::routes())
}