use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub mod models;
pub mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_db_pool() -> PgPool {
    dotenv::dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_db_connection(pool: &PgPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get().expect("Unable to connect to database")
}
