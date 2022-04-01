use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn get_db_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(50)
        .connect_timeout(std::time::Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}
