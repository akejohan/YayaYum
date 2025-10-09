use shuttle_runtime::SecretStore;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn setup_database(secrets: SecretStore) -> Result<SqlitePool, sqlx::Error> {
    let db_url = secrets
        .get("DATABASE_URL")
        .unwrap_or_else(|| "sqlite://yayayum.db".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Could not create users table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS dishes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            price_kr INTEGER NOT NULL,
            dietary_restrictions TEXT NOT NULL,
            category TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Could not create dishes table");

    Ok(pool)
}