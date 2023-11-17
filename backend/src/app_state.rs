use crate::db;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub pool: db::ConnectionPool,
    pub admin_key: String,
}

impl AppState {
    pub fn env() -> anyhow::Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .expect("missing environmental variable: DATABASE_URL");
        let pool = db::connect(&database_url)?;

        let admin_key = env::var("ADMIN_KEY")
            .expect("missing environmental variable: ADMIN_KEY");

        Ok(Self { pool, admin_key })
    }
}
