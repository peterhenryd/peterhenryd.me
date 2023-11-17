use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::Router;
use std::env;
use time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_sessions::sqlx::PgPool;
use tower_sessions::{Expiry, PostgresStore, SessionManagerLayer};
use tracing::warn;

pub async fn with_sessions(router: Router<()>) -> anyhow::Result<Router<()>> {
    let database_url = env::var("SESSION_DATABASE_URL")
        .expect("missing environmental variable: SESSION_DATABASE_URL");
    let session_pg_pool = PgPool::connect(&database_url).await?;
    let session_store = PostgresStore::new(session_pg_pool);

    session_store.migrate().await?;

    let handle_error_layer = HandleErrorLayer::new(|e: BoxError| async move {
        warn!("session error: {}", e);
        StatusCode::BAD_REQUEST
    });
    let session_manager_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_expiry(Expiry::OnInactivity(Duration::days(2)));
    let sessions = ServiceBuilder::new()
        .layer(handle_error_layer)
        .layer(session_manager_layer);

    Ok(router.layer(sessions))
}
