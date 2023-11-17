#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::panic
)]

use crate::app_state::AppState;
use crate::routes::router;
use crate::session::with_sessions;
use axum::Server;
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use tracing::{info, warn};

mod api_query;
mod app_error;
mod app_state;
mod db;
mod models;
mod responses;
mod routes;
mod schema;
mod session;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(e) = dotenv() {
        warn!("there was a problem loading the .env file: {}", e);
    }

    let socket_addr_string = env::var("SOCKET_ADDR")
        .expect("missing environmental variable: SOCKET_ADDR");
    let socket_addr = SocketAddr::from_str(&socket_addr_string)?;

    let state = AppState::env()?;
    let router = with_sessions(router(state)).await?;

    info!("starting server at {}", &socket_addr);

    Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
