use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn connect(database_url: &str) -> anyhow::Result<ConnectionPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;

    Ok(pool)
}