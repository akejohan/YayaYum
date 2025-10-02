use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Serialize, ToSchema, Deserialize, FromRow)]
pub struct User {
    pub id: u64,
    pub username: String,
}