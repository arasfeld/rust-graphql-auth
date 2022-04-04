use sqlx::postgres::{PgPool, PgPoolOptions};

lazy_static::lazy_static! {
    pub static ref DATABASE_URL: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}

pub async fn get_db_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(50)
        .connect_timeout(std::time::Duration::from_secs(3))
        .connect(&DATABASE_URL)
        .await
        .expect("Failed to create pool")
}
