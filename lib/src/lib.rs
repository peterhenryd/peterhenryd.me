use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub mod models;
pub mod schema;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_pool(database_url: impl Into<String>) -> anyhow::Result<ConnectionPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;

    Ok(pool)
}