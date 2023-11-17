use std::env;
use crate::db;

#[derive(Clone)]
pub struct AppState {
    pub pool: db::ConnectionPool,
    pub admin_key: String,
}

impl AppState {
    pub fn env() -> anyhow::Result<Self> {
        let database_url = env::var("DATABASE_URL")?;
        let pool = db::connect(&database_url)?;

        let admin_key = env::var("ADMIN_KEY")?;

        Ok(AppState { pool, admin_key })
    }
}