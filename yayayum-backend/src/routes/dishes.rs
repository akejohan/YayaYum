use axum::{routing::post, extract::{State, Path}, http::StatusCode, Json, Router};
use sqlx::{SqlitePool, Row};
use crate::models::{CreateDish, Dish};

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/dishes", post(create_dish).get(get_dishes))
        .route("/dishes/{id}", axum::routing::put(modify_dish).delete(remove_dish))
}

#[utoipa::path(
    post,
    path = "/dishes",
    request_body = CreateDish,
    responses((status = 201, description = "Dish created", body = Dish)),
    tag = "dishes"
)]
pub async fn create_dish(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateDish>,
) -> Result<(StatusCode, Json<Dish>), StatusCode> {
    // Convert enums to strings for SQLite storage
    let dietary_restrictions_json = serde_json::to_string(&payload.dietary_restrictions)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let category_str = serde_json::to_string(&payload.category)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row = sqlx::query(
        "INSERT INTO dishes (name, description, price_kr, dietary_restrictions, category) 
         VALUES (?, ?, ?, ?, ?) 
         RETURNING id, name, description, price_kr, dietary_restrictions, category"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.price_kr)
    .bind(&dietary_restrictions_json)
    .bind(&category_str)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Parse back the JSON strings to enums
    let dietary_restrictions = serde_json::from_str(&row.get::<String, _>("dietary_restrictions"))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let category = serde_json::from_str(&row.get::<String, _>("category"))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dish = Dish {
        id: row.get::<i64, _>("id") as u64,
        name: row.get("name"),
        description: row.get("description"),
        price_kr: row.get("price_kr"),
        dietary_restrictions,
        category,
    };

    Ok((StatusCode::CREATED, Json(dish)))
}

#[utoipa::path(
    get,
    path = "/dishes",
    responses((status = 200, description = "List dishes", body = [Dish])),
    tag = "dishes"
)]
pub async fn get_dishes(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Dish>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT id, name, description, price_kr, dietary_restrictions, category FROM dishes"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut dishes = Vec::new();
    for row in rows {
        let dietary_restrictions = serde_json::from_str(&row.get::<String, _>("dietary_restrictions"))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let category = serde_json::from_str(&row.get::<String, _>("category"))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        dishes.push(Dish {
            id: row.get::<i64, _>("id") as u64,
            name: row.get("name"),
            description: row.get("description"),
            price_kr: row.get("price_kr"),
            dietary_restrictions,
            category,
        });
    }

    Ok(Json(dishes))
}

#[utoipa::path(
    put,
    path = "/dishes/{id}",
    request_body = CreateDish,
    params(
        ("id" = i64, Path, description = "Dish ID to modify")
    ),
    responses(
        (status = 200, description = "Dish updated successfully", body = Dish),
        (status = 404, description = "Dish not found")
    ),
    tag = "dishes"
)]
pub async fn modify_dish(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateDish>,
) -> Result<Json<Dish>, StatusCode> {
    // Convert enums to strings for SQLite storage
    let dietary_restrictions_json = serde_json::to_string(&payload.dietary_restrictions)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let category_str = serde_json::to_string(&payload.category)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row = sqlx::query(
        "UPDATE dishes SET name = ?, description = ?, price_kr = ?, dietary_restrictions = ?, category = ? 
         WHERE id = ? 
         RETURNING id, name, description, price_kr, dietary_restrictions, category"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.price_kr)
    .bind(&dietary_restrictions_json)
    .bind(&category_str)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    // Parse back the JSON strings to enums
    let dietary_restrictions = serde_json::from_str(&row.get::<String, _>("dietary_restrictions"))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let category = serde_json::from_str(&row.get::<String, _>("category"))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dish = Dish {
        id: row.get::<i64, _>("id") as u64,
        name: row.get("name"),
        description: row.get("description"),
        price_kr: row.get("price_kr"),
        dietary_restrictions,
        category,
    };

    Ok(Json(dish))
}

#[utoipa::path(
    delete,
    path = "/dishes/{id}",
    params(
        ("id" = i64, Path, description = "Dish ID to remove")
    ),
    responses(
        (status = 204, description = "Dish deleted successfully"),
        (status = 404, description = "Dish not found")
    ),
    tag = "dishes"
)]
pub async fn remove_dish(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM dishes WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}