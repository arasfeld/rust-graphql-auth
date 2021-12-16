use crate::errors::ServiceError;

pub mod models;
pub mod schema;

type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;

pub fn get_db_pool() -> Pool {
    dotenv::dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub fn get_db_connection(pool: &Pool) -> Result<PooledConnection, ServiceError> {
    Ok(pool.get().map_err(|_| ServiceError::UnableToConnectToDb)?)
}
