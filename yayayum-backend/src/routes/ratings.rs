use axum::{extract::{Path, State}, http::StatusCode, response::Json, routing::{get, post}, Router};
use sqlx::{PgPool, Row};
use crate::models::{Rating, CreateRating};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/ratings", post(create_rating).get(get_ratings))
        .route("/ratings/{id}", get(get_rating).put(modify_rating).delete(remove_rating))
        .route("/ratings/dish/{dish_id}", get(get_ratings_by_dish))
        .route("/ratings/user/{user_id}", get(get_ratings_by_user))
}

#[utoipa::path(
    post,
    path = "/ratings",
    request_body = CreateRating,
    responses((status = 201, description = "Rating created", body = Rating)),
    tag = "ratings"
)]
pub async fn create_rating(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateRating>,
) -> Result<(StatusCode, Json<Rating>), StatusCode> {
    // Validate rating is between 1-5
    if payload.rating < 1 || payload.rating > 5 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let row = sqlx::query(
        "INSERT INTO ratings (dish_id, rating, user_id, description, photo, date) 
         VALUES ($1, $2, $3, $4, $5, NOW()) 
         RETURNING id, dish_id, rating, user_id, description, photo, date"
    )
    .bind(payload.dish_id)
    .bind(payload.rating)
    .bind(payload.user_id)
    .bind(&payload.description)
    .bind(&payload.photo)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let rating = Rating {
        id: row.get("id"),
        dish_id: row.get("dish_id"),
        rating: row.get("rating"),
        user_id: row.get("user_id"),
        description: row.get("description"),
        photo: row.get("photo"),
        date: row.get("date"),
    };

    Ok((StatusCode::CREATED, Json(rating)))
}

#[utoipa::path(
    get,
    path = "/ratings",
    responses((status = 200, description = "List ratings", body = [Rating])),
    tag = "ratings"
)]
pub async fn get_ratings(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Rating>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT id, dish_id, rating, user_id, description, photo, date FROM ratings ORDER BY date DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut ratings = Vec::new();
    for row in rows {
        ratings.push(Rating {
            id: row.get("id"),
            dish_id: row.get("dish_id"),
            rating: row.get("rating"),
            user_id: row.get("user_id"),
            description: row.get("description"),
            photo: row.get("photo"),
            date: row.get("date"),
        });
    }

    Ok(Json(ratings))
}

#[utoipa::path(
    get,
    path = "/ratings/{id}",
    params(
        ("id" = i32, Path, description = "Rating ID")
    ),
    responses(
        (status = 200, description = "Rating found", body = Rating),
        (status = 404, description = "Rating not found")
    ),
    tag = "ratings"
)]
pub async fn get_rating(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Rating>, StatusCode> {
    let row = sqlx::query(
        "SELECT id, dish_id, rating, user_id, description, photo, date FROM ratings WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let rating = Rating {
        id: row.get("id"),
        dish_id: row.get("dish_id"),
        rating: row.get("rating"),
        user_id: row.get("user_id"),
        description: row.get("description"),
        photo: row.get("photo"),
        date: row.get("date"),
    };

    Ok(Json(rating))
}

#[utoipa::path(
    get,
    path = "/ratings/dish/{dish_id}",
    params(
        ("dish_id" = i32, Path, description = "Dish ID")
    ),
    responses((status = 200, description = "Ratings for dish", body = [Rating])),
    tag = "ratings"
)]
pub async fn get_ratings_by_dish(
    State(pool): State<PgPool>,
    Path(dish_id): Path<i32>,
) -> Result<Json<Vec<Rating>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT id, dish_id, rating, user_id, description, photo, date FROM ratings WHERE dish_id = $1 ORDER BY date DESC"
    )
    .bind(dish_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut ratings = Vec::new();
    for row in rows {
        ratings.push(Rating {
            id: row.get("id"),
            dish_id: row.get("dish_id"),
            rating: row.get("rating"),
            user_id: row.get("user_id"),
            description: row.get("description"),
            photo: row.get("photo"),
            date: row.get("date"),
        });
    }

    Ok(Json(ratings))
}

#[utoipa::path(
    get,
    path = "/ratings/user/{user_id}",
    params(
        ("user_id" = i32, Path, description = "User ID")
    ),
    responses((status = 200, description = "Ratings by user", body = [Rating])),
    tag = "ratings"
)]
pub async fn get_ratings_by_user(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Rating>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT id, dish_id, rating, user_id, description, photo, date FROM ratings WHERE user_id = $1 ORDER BY date DESC"
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut ratings = Vec::new();
    for row in rows {
        ratings.push(Rating {
            id: row.get("id"),
            dish_id: row.get("dish_id"),
            rating: row.get("rating"),
            user_id: row.get("user_id"),
            description: row.get("description"),
            photo: row.get("photo"),
            date: row.get("date"),
        });
    }

    Ok(Json(ratings))
}

#[utoipa::path(
    put,
    path = "/ratings/{id}",
    params(
        ("id" = i32, Path, description = "Rating ID to update")
    ),
    request_body = CreateRating,
    responses(
        (status = 200, description = "Rating updated", body = Rating),
        (status = 404, description = "Rating not found")
    ),
    tag = "ratings"
)]
pub async fn modify_rating(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateRating>,
) -> Result<Json<Rating>, StatusCode> {
    // Validate rating is between 1-5
    if payload.rating < 1 || payload.rating > 5 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let row = sqlx::query(
        "UPDATE ratings SET dish_id = $1, rating = $2, user_id = $3, description = $4, photo = $5 
         WHERE id = $6 
         RETURNING id, dish_id, rating, user_id, description, photo, date"
    )
    .bind(payload.dish_id)
    .bind(payload.rating)
    .bind(payload.user_id)
    .bind(&payload.description)
    .bind(&payload.photo)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let rating = Rating {
        id: row.get("id"),
        dish_id: row.get("dish_id"),
        rating: row.get("rating"),
        user_id: row.get("user_id"),
        description: row.get("description"),
        photo: row.get("photo"),
        date: row.get("date"),
    };

    Ok(Json(rating))
}

#[utoipa::path(
    delete,
    path = "/ratings/{id}",
    params(
        ("id" = i32, Path, description = "Rating ID to remove")
    ),
    responses(
        (status = 204, description = "Rating deleted"),
        (status = 404, description = "Rating not found")
    ),
    tag = "ratings"
)]
pub async fn remove_rating(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM ratings WHERE id = $1")
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