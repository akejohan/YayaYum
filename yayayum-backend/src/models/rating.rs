use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Deserialize, FromRow)]
pub struct Rating {
    pub id: i32,
    pub dish_id: i32,
    pub rating: i32, // 1-5
    pub user_id: i32,
    pub description: Option<String>,
    pub photo: Option<String>, // URL or path to photo
    #[schema(value_type = String, format = "date-time")]
    pub date: chrono::NaiveDateTime,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateRating {
    pub dish_id: i32,
    pub rating: i32, // 1-5
    pub user_id: i32,
    pub description: Option<String>,
    pub photo: Option<String>, // URL or path to photo
}