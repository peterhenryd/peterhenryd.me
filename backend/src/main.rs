#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
)]

use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use axum::Server;
use dotenvy::dotenv;
use tracing::{info, warn};
use crate::routes::router;
use crate::session::with_sessions;
use crate::app_state::AppState;

mod app_state;
mod routes;
mod db;
mod app_error;
mod schema;
mod models;
mod session;
mod api_query;
mod responses;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(e) = dotenv() {
        warn!("There was a problem loading the .env file: {}", e);
    }

    let socket_addr_string = env::var("SOCKET_ADDR")?;
    let socket_addr = SocketAddr::from_str(&socket_addr_string)?;

    let state = AppState::env()?;

    info!("Starting server at {}.", &socket_addr);

    let router = with_sessions(router(state)).await?;

    Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
