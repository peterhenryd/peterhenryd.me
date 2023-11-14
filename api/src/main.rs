use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use axum::{Router, Server};
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use dotenvy::dotenv;
use tower::{BoxError, ServiceBuilder};
use tower::layer::util::{Identity, Stack};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tower_sessions::cookie::time::Duration;
use tracing::info;
use peterhenryd_me_lib::{ConnectionPool, get_connection_pool};
use crate::state::AppState;

mod state;
mod handlers;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(e) = dotenv() {
        info!("Error loading .env file: {}", e);
    }

    let socket_addr = SocketAddr::from_str(&env::var("SOCKET_ADDR")
        .expect("Missing environmental variable SOCKET_ADDR"))?;

    let pool = get_connection_pool(env::var("DATABASE_URL")
        .expect("Missing environmental variable DATABASE_URL"))?;

    info!("Starting server at address {}.", &socket_addr);

    Server::bind(&socket_addr)
        .serve(router(pool).into_make_service())
        .await?;

    Ok(())
}

/// API description:
///   GET    /blog-posts/all     Returns all the blog posts in JSON.
///   GET    /blog-posts/id/:id  Returns the blog post with the specified id in JSON.
///   GET    /users/session      Returns the current user in the session.
///   POST   /users/session      Attempts to login the user.
///   DELETE /users/session      Logs the user out.
///   POST   /users/new          Creates a new user.
///   GET    /users/all          Admin only: Returns all the users in JSON.
///   GET    /users/id/:id       Admin only: Returns the user with the specified id in JSON.
fn router(pool: ConnectionPool) -> Router<()> {
    let sessions = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| StatusCode::BAD_REQUEST))
        .layer(SessionManagerLayer::new(MemoryStore::default())
            .with_secure(true)
            .with_expiry(Expiry::OnInactivity(Duration::days(2)))
        );

    Router::new()
        .route("/blog-posts/all", get(handlers::blog_posts::load_all_handler))
        .route("/blog-posts/id/:id", get(handlers::blog_posts::load_by_id_handler))
        .route("/users/session", get(handlers::users::fetch_session_handler))
        .route("/users/session", post(handlers::users::create_session_handler))
        .route("/users/session", delete(handlers::users::delete_session_handler))
        .route("/users/new", post(handlers::users::create_handler))
        .route("/users/all", delete(handlers::users::load_all_handler))
        .route("/users/id/:id", delete(handlers::users::load_by_id_handler))
        .with_state(AppState { pool })
        .layer(sessions)
}