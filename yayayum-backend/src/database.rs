use shuttle_runtime::SecretStore;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn setup_database(secrets: SecretStore) -> Result<PgPool, sqlx::Error> {
    let db_url = secrets
        .get("DATABASE_URL")
        .unwrap_or_else(|| "postgres://postgres:password@localhost/yayayum".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    Ok(pool)
}