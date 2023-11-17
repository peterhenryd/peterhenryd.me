use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect(database_url: &str) -> anyhow::Result<ConnectionPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;

    Ok(pool)
}
