use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, time::Duration};

pub async fn create_pool() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(50) // Increase to 50-100 depending on your infrastructure
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(1800)) // 30 minutes
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres")
}
